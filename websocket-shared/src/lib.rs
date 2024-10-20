use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]

pub struct LeaderboardElos(Vec<LeaderboardEloEntry>);

impl LeaderboardElos {
    pub fn new(values: Vec<LeaderboardEloEntry>) -> Self {
        Self(values)
    }
}

impl Deref for LeaderboardElos {
    type Target = Vec<LeaderboardEloEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LeaderboardElos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OutgoingMessage {
    InitialLeaderboards {
        leaderboards: HashMap<LeaderboardName, LeaderboardElos>,
    },
    Changes {
        changes: LeaderboardsChanges,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LeaderboardPosistion(usize);

impl LeaderboardPosistion {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LeaderboardEloChanges(HashMap<LeaderboardPosistion, LeaderboardEloEntry>);

impl LeaderboardEloChanges {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for LeaderboardEloChanges {
    type Target = HashMap<LeaderboardPosistion, LeaderboardEloEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LeaderboardEloChanges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LeaderboardsChanges(HashMap<LeaderboardName, LeaderboardEloChanges>);

impl LeaderboardsChanges {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self) -> &HashMap<LeaderboardName, LeaderboardEloChanges> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut HashMap<LeaderboardName, LeaderboardEloChanges> {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LeaderboardName(String);

impl LeaderboardName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Elo(f32);

impl Elo {
    pub fn new(value: f32) -> Self {
        if !value.is_finite() {
            panic!("elo value was not finite: {value:?}");
        }
        Self(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaderboardEloEntry {
    pub author_id: AuthorId,
    pub elo: Elo,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TwitchId(String);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiscordId(String);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct B2Id(String);

impl TwitchId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl DiscordId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl B2Id {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "platform", content = "id", rename_all = "snake_case")]
pub enum AuthorId {
    Twitch(TwitchId),
    Discord(DiscordId),
    B2(B2Id),
}
