use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};
use twitch_utils::TwitchAPIWrapper;

pub async fn handle_messages(
    sender: UnboundedSender<elo::_types::clptypes::Message>,
    twitch_api_wrapper: TwitchAPIWrapper,
) {
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, _>::new(twitch_irc::ClientConfig {
            login_credentials: StaticLoginCredentials::anonymous(),
            ..Default::default()
        });

    // TODO: put this in a constant
    let ved_ch_name = "vedal987";

    client
        .join(ved_ch_name.to_string())
        .expect("failed to join twitch chat");

    let mut avatars = HashMap::new();
    let mut emotes = HashSet::new();

    loop {
        let twitch_irc::message::ServerMessage::Privmsg(privmmsg) =
            incoming_messages.recv().await.unwrap()
        else {
            continue;
        };

        if !avatars.contains_key(&privmmsg.sender.id) {
            let avatar = twitch_api_wrapper
                .twitch
                .get_user_from_id(&privmmsg.sender.id, &twitch_api_wrapper.token)
                .await
                .unwrap()
                .unwrap()
                .profile_image_url
                .unwrap();
            avatars.insert(privmmsg.sender.id.clone(), avatar);
        }
        let avatar = avatars.get(&privmmsg.sender.id).unwrap();

        privmmsg.emotes.iter().for_each(|e| {
            emotes.insert(e.code.clone());
        });

        let comment = twitch_utils::twitchtypes::Comment {
            _id: privmmsg.message_id,
            message: twitch_utils::twitchtypes::ChatMessage {
                body: privmmsg.message_text.clone(),
                bits_spent: privmmsg.bits.unwrap_or(0) as u32,
                fragments: message_to_fragments(&privmmsg.message_text, &emotes)
                    .into_iter()
                    .map(|f| f.into())
                    .collect(),
                user_badges: Some(
                    privmmsg
                        .badges
                        .iter()
                        .map(|badge| twitch_utils::twitchtypes::Badge {
                            _id: badge.name.clone(),
                            version: badge.version.clone(),
                        })
                        .collect(),
                ),
            },
            commenter: twitch_utils::twitchtypes::ChatUserInfo {
                display_name: privmmsg.sender.name,
                _id: privmmsg.sender.id,
                logo: avatar.to_string(),
            },
        };

        sender
            .send(elo::_types::clptypes::Message::Twitch(comment))
            .unwrap();
    }
}

use std::collections::{HashMap, HashSet};

pub fn message_to_fragments(message: &str, emotes: &HashSet<String>) -> Vec<Fragment> {
    let mut framgents = Vec::new();

    let mut current_text = String::new();

    for word in message.split(" ") {
        if emotes.contains(word) {
            if !current_text.is_empty() {
                current_text.push(' ');
                framgents.push(Fragment::Text(std::mem::take(&mut current_text)));
            }

            if framgents
                .last()
                .map(|f| match f {
                    Fragment::Text(_) => false,
                    Fragment::Emote(_) => true,
                })
                .unwrap_or(false)
            {
                framgents.push(Fragment::Text(" ".to_string()))
            }

            framgents.push(Fragment::Emote(word.to_string()));
        } else {
            if !framgents.is_empty() || !current_text.is_empty() {
                current_text.push(' ');
            }

            current_text.push_str(word);
        }
    }

    if !current_text.is_empty() {
        if !framgents.is_empty() {
            current_text.push(' ');
        }

        framgents.push(Fragment::Text(std::mem::take(&mut current_text)));
    }

    framgents
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    Text(String),
    Emote(String),
}

impl Fragment {
    fn text(&self) -> &str {
        match self {
            Fragment::Text(text) => text,
            Fragment::Emote(emote) => emote,
        }
    }
}

impl From<Fragment> for twitch_utils::twitchtypes::ChatMessageFragment {
    fn from(value: Fragment) -> Self {
        twitch_utils::twitchtypes::ChatMessageFragment {
            text: value.text().to_string(),
            emoticon: match value {
                Fragment::Text(_) => None,
                Fragment::Emote(emote) => {
                    Some(twitch_utils::twitchtypes::ChatMessageFragmentEmoticon {
                        emoticon_id: emote,
                    })
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::provider::twitch::{message_to_fragments, Fragment};

    #[test]
    fn message_with_emotes() {
        let input = "poggers im pogging to this poggers stream, i just love saying poggers poggers";
        let fragments = message_to_fragments(&input, &HashSet::from(["poggers".to_string()]));
        let poggers = Fragment::Emote("poggers".to_string());
        assert_eq!(
            vec![
                poggers.clone(),
                Fragment::Text(" im pogging to this ".to_string()),
                poggers.clone(),
                Fragment::Text(" stream, i just love saying ".to_string()),
                poggers.clone(),
                Fragment::Text(" ".to_string()),
                poggers.clone(),
            ],
            fragments,
            "fragments did not patch"
        );
    }

    #[test]
    fn message_no_emotes() {
        let input = "this is a normal message";
        let fragments = message_to_fragments(&input, &HashSet::new());
        assert_eq!(
            vec![Fragment::Text("this is a normal message".to_string())],
            fragments,
            "fragments did not patch"
        );
    }

    #[test]
    fn just_one_emote() {
        let input = "hi poggers";
        let fragments = message_to_fragments(&input, &HashSet::from(["poggers".to_string()]));
        assert_eq!(
            vec![
                Fragment::Text("hi ".to_string()),
                Fragment::Emote("poggers".to_string()),
            ],
            fragments,
            "fragments did not patch"
        );
    }
}
