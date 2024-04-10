use bevy::prelude::Component;

pub enum HostileType {
    Zombie,
}

pub enum FriendlyType {
    Chicken,
}

pub enum HostileMobState {
    Idle,
    AttackingPlayer,
}

pub enum FriendlyMobState {
    Idle,
    Eating,
    Running,
}


#[derive(Component)]
pub struct HostileMob {
    pub mob_type: HostileType,
    pub state: HostileMobState,
}

#[derive(Component)]
pub struct FriendlyMob {
    pub mob_type: FriendlyType,
    pub state: FriendlyMobState,
}
