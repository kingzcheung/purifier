
pub(crate) const data: [(&str, &str);19] = [
    (
        r#"sdfs json```{ s {"name":"沙滩上的女子和狗","category":"宠物用品","packaging":"","tags":["海滩","户外活动"],"compositions":[],"description":"这张照片展示了一位女士在海滩上享受时光时与她的狗狗互动的情景。","shooting_angle":""}```ssdf"#,
        r#"{"name":"沙滩上的女子和狗","category":"宠物用品","packaging":"","tags":["海滩","户外活动"],"compositions":[],"description":"这张照片展示了一位女士在海滩上享受时光时与她的狗狗互动的情景。","shooting_angle":""}"#),
    (
        r#"kjsfhd { "event": "音乐会", "location": "皇家音乐厅", "date": "2023-04-15", "performers": ["交响乐团", "独奏家：莉莉·陈"] } jfdsklf"#,
        r#"{"event":"音乐会","location":"皇家音乐厅","date":"2023-04-15","performers":["交响乐团","独奏家：莉莉·陈"]}"#
    ),
    (
        r#"sfjls { "product": "复古相机", "manufacturer": "经典映像", "features": ["全手动操作", "胶片支持"], "status": "缺货" } sdfkj"#,
        r#"{"product":"复古相机","manufacturer":"经典映像","features":["全手动操作","胶片支持"],"status":"缺货"}"#
    ),
    (
        r#"asf { "recipe": { "title": "番茄炒蛋", "ingredients": ["鸡蛋", "番茄", "盐"], "steps": ["打蛋", "切番茄", "快炒"] }, "difficulty": "简单" } asdf"#,
        r#"{"recipe":{"title":"番茄炒蛋","ingredients":["鸡蛋","番茄","盐"],"steps":["打蛋","切番茄","快炒"]},"difficulty":"简单"}"#
    ),
    (
        r#"sdf { "travel_plan": { "destination": "京都", "activities": ["参观金阁寺", "体验茶道"], "hotel": "和风旅馆" }, "departure_date": "2023-05-01" } sdfg"#,
        r#"{"travel_plan":{"destination":"京都","activities":["参观金阁寺","体验茶道"],"hotel":"和风旅馆"},"departure_date":"2023-05-01"}"#
    ),
    (
        r#"#@$%~^&*()_+=-{}|[]:";'<>?,./`~!@#$%^&*(\n`{ "galactic_news": { "headline": "新星系发现!", "source": "宇宙探索局", "date": "2023-04-25T12:00:00Z" } }\n`~!@#$%^&*()"#,
        r#"{"galactic_news":{"headline":"新星系发现!","source":"宇宙探索局","date":"2023-04-25T12:00:00Z"}}"#
    ),
    (
        r#"/*-+}{]["';:.>,<?/|\n`~`` { "historical_event": { "year": 1789, "event": "法国大革命爆发", "key_figures": ["罗伯斯庇尔", "玛丽·安托瓦内特"] } } `~-!@#$%^&*("#,
        r#"{"historical_event":{"year":1789,"event":"法国大革命爆发","key_figures":["罗伯斯庇尔","玛丽·安托瓦内特"]}}"#
    ),
    (
        r#"//__++--==[]{};:'",.<>?/|\|`~_ { "daily_horoscope": { "sign": "双子座", "prediction": "今日宜出行，注意人际沟通。" } } `~~!@#$%^&*()"#,
        r#"{"daily_horoscope":{"sign":"双子座","prediction":"今日宜出行，注意人际沟通。"}}"#
    ),
    (
        r#"/*-+}{]["';:.>,<?/|\n`~`` { "recipe": { "dish": "麻辣香锅", "ingredients": ["牛肉", "莲藕", "金针菇"], "spiciness_level": 5 } } `~-!@#$%^&*("#,
        r#"{"recipe":{"dish":"麻辣香锅","ingredients":["牛肉","莲藕","金针菇"],"spiciness_level":5}}"#
    ),
    (
        r#"`~/|}{][":;,'.<>?/|\n`~`` { "movie": { "title": "盗梦空间", "director": "克里斯托弗·诺兰", "release_year": 2010, "genre": "科幻" } } `~!@#$%^&*()"#,
        r#"{"movie":{"title":"盗梦空间","director":"克里斯托弗·诺兰","release_year":2010,"genre":"科幻"}}"#
    ),
    (
        r#"/*-+}{]["';:.>,<?/|\n`~`` { "tech_innovation": { "name": "量子计算机原型", "developer": "量子未来实验室", "announced_on": "2023-05-15" } } `~-!@#$%^&*("#,
        r#"{"tech_innovation":{"name":"量子计算机原型","developer":"量子未来实验室","announced_on":"2023-05-15"}}"#
    ),
    (
        r#"`~/|}{][":;,'.<>?/|\n`~`` { "space_mission": { "name": "火星样本返回任务", "agency": "NASA", "launch_date": "2026-07-14", "status": "计划中" } } `~!@#$%^&*()"#,
        r#"{"space_mission":{"name":"火星样本返回任务","agency":"NASA","launch_date":"2026-07-14","status":"计划中"}}"#
    ),
    (
        r#"//__++--==[]{};:'",.<>?/|\|`~_ { "architectural_wonder": { "name": "哈利法塔", "location": "迪拜", "height_meters": 828, "completed_year": 2010 } } `~~!@#$%^&*()"#,
        r#"{"architectural_wonder":{"name":"哈利法塔","location":"迪拜","height_meters":828,"completed_year":2010}}"#
    ),
    (
        r#"测试数据21:
/*-+}{]["';:.>,<?/|\n`~`` { 
    "user_profile": {
        "username": "SapphireGazer",
        "joined": "2018-09-15T08:00:00Z",
        "preferences": {
            "theme": "dark",
            "notifications": {
                "email": true,
                "sms": false
            }
        },
        "recent_activity": [
            { "action": "post", "timestamp": "2023-04-12T14:30:00Z", "content": "分享了一篇关于AI伦理的文章" },
            { "action": "comment", "timestamp": "2023-04-10T22:00:00Z", "content": "参与了关于未来教育的讨论" }
        ]
    }
} `~-!@#$%^&*("#,
        r#"{"user_profile":{"username":"SapphireGazer","joined":"2018-09-15T08:00:00Z","preferences":{"theme":"dark","notifications":{"email":true,"sms":false}},"recent_activity":[{"action":"post","timestamp":"2023-04-12T14:30:00Z","content":"分享了一篇关于AI伦理的文章"},{"action":"comment","timestamp":"2023-04-10T22:00:00Z","content":"参与了关于未来教育的讨论"}]}}"#
    ),
    (
        r#"测试数据22:
//__++--==[]{};:'",.<>?/|\|`~_ { 
    "financial_report": {
        "quarter": "Q1",
        "year": 2023,
        "revenue": {
            "total": 12345678.90,
            "by_product": {
                "product_A": 456789.00,
                "product_B": 3456789.00
            }
        },
        "expenses": {
            "operating": 234567.89,
            "marketing": 123456.78
        },
        "profit_margin": "25%"
    }
} `~~!@#$%^&*()"#,
        r#"{"financial_report":{"quarter":"Q1","year":2023,"revenue":{"total":12345678.9,"by_product":{"product_A":456789,"product_B":3456789}},"expenses":{"operating":234567.89,"marketing":123456.78},"profit_margin":"25%"}}"#
    ),
    (
        r#"测试数据23:
`~/|}{][":;,'.<>?/|\n`~`` { 
    "scientific_study": {
        "title": "基因编辑技术在疾病治疗中的应用",
        "authors": ["张华", "李明", "王丽"],
        "publication_date": "2023-03-20",
        "results": {
            "positive_outcomes": ["治疗效率提升30%", "副作用减少"],
            "challenges": ["长期安全性评估", "伦理争议"]
        },
        "methods": {
            "CRISPR-Cas9": {
                "description": "一种精确的基因编辑工具",
                "application_cases": ["遗传性疾病", "癌症研究"]
            }
        }
    }
} `~!@#$%^&*()"#,
        r#"{"scientific_study":{"title":"基因编辑技术在疾病治疗中的应用","authors":["张华","李明","王丽"],"publication_date":"2023-03-20","results":{"positive_outcomes":["治疗效率提升30%","副作用减少"],"challenges":["长期安全性评估","伦理争议"]},"methods":{"CRISPR-Cas9":{"description":"一种精确的基因编辑工具","application_cases":["遗传性疾病","癌症研究"]}}}}"#
    ),
    (
        r#"测试数据24:
/*-+}{]["';:.>,<?/|\n`~`` { 
    "travel_plan": {
        "destination": "京都，日本",
        "start_date": "2023-10-01",
        "end_date": "2023-10-10",
        "accommodation": {
            "hotel_name": "樱花旅馆",
            "check_in": "2023-10-01",
            "check_out": "2023-10-10"
        },
        "itinerary": [
            { "day": 1, "activity": "参观金阁寺", "notes": "不要错过夕阳下的景色" },
            { "day": 3, "activity": "体验茶道课程", "notes": "预订已确认，10:00开始" }
        ],
        "budget": {
            "flights": 500.00,
            "accommodation": 800.00,
            "activities": 300.00
        }
    }
} `~-!@#$%^&*("#,
        r#"{"travel_plan":{"destination":"京都，日本","start_date":"2023-10-01","end_date":"2023-10-10","accommodation":{"hotel_name":"樱花旅馆","check_in":"2023-10-01","check_out":"2023-10-10"},"itinerary":[{"day":1,"activity":"参观金阁寺","notes":"不要错过夕阳下的景色"},{"day":3,"activity":"体验茶道课程","notes":"预订已确认，10:00开始"}],"budget":{"flights":500,"accommodation":800,"activities":300}}}"#
    ),
    (
        r#"[]{};:'",.<>?/|\|`~_ { 
    "iot_device_configuration": {
        "device_id": "DVC-XYZ123",
        "model": "SmartHome Hub v3",
        "firmware_version": "2.5.1",
        "settings": {
            "network": {
                "ssid": "MyHomeWiFi",
                "password": "SecureP@ss123"
            },
            "security": {
                "firewall": true,
                "auto_updates": true
            },
            "sensors": [
                { "type": "temperature", "interval": "5m" },
                { "type": "humidity", "interval": "10m" }
            ]
        },
        "last_update": "2023-04-20T15:30:00Z"
    }
} `~~!@#$%^&*()"#,
        r#"{"iot_device_configuration":{"device_id":"DVC-XYZ123","model":"SmartHome Hub v3","firmware_version":"2.5.1","settings":{"network":{"ssid":"MyHomeWiFi","password":"SecureP@ss123"},"security":{"firewall":true,"auto_updates":true},"sensors":[{"type":"temperature","interval":"5m"},{"type":"humidity","interval":"10m"}]},"last_update":"2023-04-20T15:30:00Z"}}"#
    ),
    (
        r#"测试数据15:
/*-+}{]["';:.>,<?/|\n`~`` { "recipe": { "dish": "麻辣香锅", "ingredients": ["牛肉", "莲藕", "金针菇"], "spiciness_level": 5 } } `~-!@#$%^&*("#,
        r#"{"recipe":{"dish":"麻辣香锅","ingredients":["牛肉","莲藕","金针菇"],"spiciness_level":5}}"#
    ),
    // (r#"tion":{"device_id":"DVC-XYZ123","model":"SmartHome "#,"")
];