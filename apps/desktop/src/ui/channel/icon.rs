use gpui_component::IconName;
use mattermost::client::channel::ChannelType;

pub fn channel_type_to_icon(channel_type: ChannelType) -> IconName {
    match channel_type {
        ChannelType::Open => IconName::Globe,
        ChannelType::Private => IconName::Star,
        ChannelType::Direct => IconName::User,
        ChannelType::Group => IconName::CircleUser,
    }
}
