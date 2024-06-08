use std::fmt::format;
use std::sync::Arc;
use bytes::{Buf, Bytes};
use log::warn;
use prost::Message;
use ntrim_tools::cqp::{At, Face, Image};
pub use ntrim_tools::cqp::CQCode;
use crate::bot::Bot;
use crate::pb::msg::elem::AioElem;
use crate::pb::msg::{ * };
use crate::pb::msg::common_elem::{CommonBigFaceElem, CommonFaceElem};
use crate::pb::trpc::olpush::{ * };
use crate::servlet::olpush::msg::{Contact, MessageRecord};

pub(crate) async fn parse_elements(bot: &Arc<Bot>, record: &mut MessageRecord, elems: Vec<Elem>) {
    let mut single_element = false;
    let result = &mut record.elements;
    for elem in elems {
        if elem.aio_elem.is_none() {
            warn!("Unsupported elem found, skip this!");
            continue;
        }
        let elem = elem.aio_elem.unwrap();
        // 也就解析个特殊头衔
        if let AioElem::Extra(ExtraData { group_nick: _group_nick, unique_title }) = elem {
            if let Some(title) = unique_title {
                record.sender_unique_title = title;
            }
            continue
        }
        // 毛都没有，就堆气泡什么的
        if let AioElem::GeneralFlags(GeneralFlags { .. }) = elem {
            continue
        }
        // 同上
        if let AioElem::Flags2(ElemFlags2 { .. }) = elem {
            continue
        }

        // 这类消息都会带一个text，提示版本过低什么的，所以说一旦出现这种消息，后续的消息都不用解析了
        if single_element {
           continue;
        }

        match elem {
            AioElem::Text(Text { text, attr_6, .. }) => {
                // attr_6是一个Option<Bytes>，如果有值，那么就是一个At，否则就是一个普通的文本消息
                if attr_6.is_some() {
                    let mut buf = Bytes::from(attr_6.unwrap());
                    let size = buf.get_u16();
                    let pos = buf.get_u16();
                    let nick_len = buf.get_u16();
                    let is_at_all = buf.get_u8();
                    let uin = buf.get_u32() as u64;
                    result.push(CQCode::Special(Box::new(At {
                        qq: uin,
                        #[cfg(feature = "extend_cqcode")]
                        content: text.clone(),
                    })))
                } else {
                    result.push(CQCode::Text(text));
                }
            }

            AioElem::NotOnlineImage(image) => {
                let md5 = hex::encode(image.pic_md5).to_uppercase();
                let url = format!(
                    "https://c2cpicdw.qpic.cn{}",
                    image.orig_url.unwrap_or(format!("/offpic_new/0/0-0-{}/0?term=2", md5).to_string())
                );
                result.push(CQCode::Special(Box::new(Image::new(
                    image.file_path.map_or(md5 + ".jpg", |v| {
                        v.replace("{", "").replace("}", "").replace("-", "")
                    }), url, image.original.unwrap_or(false)
                ))));
            }

            AioElem::CustomFace(image) => {
                let md5 = hex::encode(image.pic_md5).to_uppercase();
                let url = format!("https://{}{}", match record.contact {
                    Contact::Group(..) => "gchat.qpic.cn",
                    Contact::Friend(..) | Contact::Stranger(..) => "c2cpicdw.qpic.cn",
                }, image.original_url.unwrap_or(match record.contact {
                    Contact::Group(..) => format!("/gchatpic_new/0/0-0-{}/0?term=2", md5),
                    Contact::Friend(..) | Contact::Stranger(..) => format!("/offpic_new/0/0-0-{}/0?term=2", md5),
                }.to_string()));
                result.push(CQCode::Special(Box::new(Image::new(
                    image.file_path.map_or(md5 + ".png", |v| {
                        v.replace("{", "").replace("}", "").replace("-", "")
                    }), url, image.original != Some(0)
                ))));
            }


            AioElem::ArkJson(LightArk { data }) => {
                warn!("Unsupported ArkJson")
            }

            AioElem::CommonElem(CommonElem{ service_type, data, business_type }) => {
                let data = Bytes::from(data);
                if service_type == 3 { // 闪照

                } else if service_type == 33 { // 那种表情消息，扩展出来的
                    let comm_face = CommonFaceElem::decode(data).unwrap();
                    result.push(CQCode::Special(Box::new(Face::new(
                        comm_face.face_id
                    ))));
                } else if service_type == 37 { // 那种大的表情消息
                    single_element = true;
                    result.clear();
                    let big_face = CommonBigFaceElem::decode(data).unwrap();
                    result.push(CQCode::Special(Box::new(Face::new_big_face(
                        big_face.face_id
                    ))));
                } else if service_type == 48 { // 新版本专属的图片推送
                    let msg_info = MsgInfo::decode(data).unwrap();
                    parse_comm_elem_48(bot, result, business_type.unwrap(), msg_info).await;
                } else {
                    warn!("Unsupported CommonElem: {}", service_type)
                }
            }
            _ => warn!("Unsupported elem found, skip this!")
        }
    }
}

pub async fn parse_comm_elem_48(bot: &Arc<Bot>, result: &mut Vec<CQCode>, business_type: u32, msg_info: MsgInfo) {
    for msg in msg_info.msg_info_body.iter() {
        let index = &msg.index;
        let upload_time = index.upload_time;
        let file_info = &index.file_info;
        let file_uuid = &index.file_uuid;
        let file_type = file_info.file_type.file_type;
        if file_type == 1 {
            let picture = msg.picture.as_ref().unwrap();
            let url_path = &picture.url_path;
            let domain = picture.domain.as_ref()
                .map_or("multimedia.nt.qq.com".to_string(), |v| v.clone());
            let rkey = crate::service::rich_media::get_download_reky(&bot, business_type as u8).await;
            let sub_type = msg_info.ext_info.as_ref()
                .map_or(0, |v| v.pic.as_ref()
                    .map_or(0, |pic| pic.biz_type.unwrap_or(0))
                );
            if let Ok(Some(rkey)) = rkey {
                let url = format!("https://{}{}&spec=0{}", domain, url_path, rkey.key);
                result.push(CQCode::Special(Box::new(Image::with_sub_type(
                    file_info.file_name.clone(), url, sub_type
                ))));
            } else {
                warn!("Failed to get download rkey for picture: {:?}", rkey);
            }
        } else {
            warn!("Unsupported file type: {}", file_type)
        }
    }
}