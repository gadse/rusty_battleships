pub enum Request {
    GetFeatures,
    Login {
        username:String
    },
    Ready,
    NotReady,
    ChallengePlayer {
        username:String
    },
    PlaceShips {
        placement:[ShipPlacement; 5]
    },
    Shoot {
        x:u8,
        y:u8,
    },
    MoveAndShoot {
        id:u8,
        direction:Direction,
        x:u8,
        y:u8,
    },
    Surrender,
    ReportError {
        errormessage:String,
    },
}

pub enum Response {
    Ok,
    Features {
        numfeatures:u8,
        features:Vec<String>,
    },
    NameTaken{
        nickname:String,
    },
    NoSuchPlayer{
        nickname:String,
    },
    NotWaiting{
        nickname:String,
    },
    GameAlreadyStarted,
    IllegalPlacement,
    Hit {
        x:u8,
        y:u8,
    },
    Miss {
        x:u8,
        y:u8,
    },
    Destroyed {
        x:u8,
        y:u8,
    },
    InvalidRequest,
}

pub enum Update {
    PlayerJoined {
        nickname:String,
    },
    PlayerLeft {
        nickname:String,
    },
    PlayerReady {
        nickname:String,
    },
    PlayerNotReady {
        nickname:String,
    },
    GameStart {
        nickname:String, //Opponent's name
    },
    YourTurn,
    EnemyTurn,
    EnemyVisible {
        x:u8,
        y:u8,
    },
    EnemyInvisible {
        x:u8,
        y:u8,
    },
    EnemyHit {
        x:u8,
        y:u8,
    },
    EnemyMiss {
        x:u8,
        y:u8,
    },
    GameOver {
        victorious:bool,
        reason:Reason,
    },
    AfkWarning {
        strikes:u8,
    },
    EnemyAfkWarning {
        strikes:u8,
    },
    ServerGoingDown,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn get_direction(direction_indicator: u8) -> Direction {
    areturn match direction_indicator {
        0: Direction::North,
        1: Direction::East,
        2: Direction::South,
        3: Direction::West,
        _: panic!()
    }
}

pub struct ShipPlacement {
    x:u8,
    y:u8,
    direction:Direction
}

pub enum Reason {
    Obliterated,
    Disconnected,
    Surrendered,
    Afk
}
