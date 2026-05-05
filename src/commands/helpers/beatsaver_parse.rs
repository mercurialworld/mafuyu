use phf::phf_map;

static KNOWN_SNS: phf::Map<&str, &str> = phf_map! {
    "twt" => "[twt@$2](https://twitter.com/$2)",
    "yt" => "[yt@$2](https://youtube.com/channel/$2)",
    "yth" => "[yth@$2](https://youtube.com/$2)",
    "ttv" => "[ttv@$2](https://twitch.tv/$2)",
    "steam" => "[steam@$2](https://steamcommunity.com/profiles/$2)",
    "ss" => "[ss@$2](https://scoresaber.com/u/$2)",
    "bl" => "[bl@$2](https://beatleader.com/u/$2)",
    "gh" => "[gh@$2](https://github.com/$2)",
};

pub fn parse_user(description: &str) {}
