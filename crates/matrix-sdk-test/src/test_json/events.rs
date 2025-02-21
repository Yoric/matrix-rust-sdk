use once_cell::sync::Lazy;
use serde_json::{json, Value as JsonValue};

pub static ALIAS: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "alias": "#tutorial:localhost"
        },
        "event_id": "$15139375513VdeRF:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.canonical_alias",
        "unsigned": {
            "age": 703422
        }
    })
});

pub static ALIASES: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "aliases": [
                "#tutorial:localhost"
            ]
        },
        "event_id": "$15139375516NUgtD:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "localhost",
        "type": "m.room.aliases",
        "unsigned": {
            "age": 703422
        }
    })
});

pub static CREATE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "creator": "@example:localhost",
            "m.federate": true,
            "room_version": "1"
        },
        "event_id": "$151957878228ekrDs:localhost",
        "origin_server_ts": 15195787,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.create",
        "unsigned": {
          "age": 139298
        }
    })
});

pub static FULLY_READ: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "event_id": "$someplace:example.org"
        },
        "room_id": "!somewhere:example.org",
        "type": "m.fully_read"
    })
});

pub static HISTORY_VISIBILITY: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "history_visibility": "world_readable"
        },
        "event_id": "$151957878235ricnD:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.history_visibility",
        "unsigned": {
          "age": 1392989
        }
    })
});

pub static JOIN_RULES: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "join_rule": "public"
        },
        "event_id": "$151957878231iejdB:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.join_rules",
        "unsigned": {
          "age": 1392989
        }
    })
});

pub static ROOM_MESSAGES: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "chunk": [
          {
            "age": 1042,
            "content": {
              "body": "hello world",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Caaaa:example.com",
            "origin_server_ts": 1444812213737i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@alice:example.com",
            "type": "m.room.message"
          },
          {
            "age": 20123,
            "content": {
              "body": "the world is big",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Cbbbb:example.com",
            "origin_server_ts": 1444812194656i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "type": "m.room.message"
          },
          {
            "age": 50789,
            "content": {
              "name": "New room name"
            },
            "event_id": "$1444812213350496Ccccc:example.com",
            "origin_server_ts": 1444812163990i64,
            "prev_content": {
              "name": "Old room name"
            },
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "state_key": "",
            "type": "m.room.name"
          }
        ],
        "end": "t47409-4357353_219380_26003_2265",
        "start": "t47429-4392820_219380_26003_2265"
    })
});

pub static SYNC_ROOM_MESSAGES_BATCH_1: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "chunk": [
          {
            "age": 1042,
            "content": {
              "body": "hello world",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Caaaf:example.com",
            "origin_server_ts": 1444812213737i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@alice:example.com",
            "type": "m.room.message"
          },
          {
            "age": 20123,
            "content": {
              "body": "the world is big",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Cbbbf:example.com",
            "origin_server_ts": 1444812194656i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "type": "m.room.message"
          },
          {
            "age": 50789,
            "content": {
              "name": "New room name"
            },
            "event_id": "$1444812213350496Ccccf:example.com",
            "origin_server_ts": 1444812163990i64,
            "prev_content": {
              "name": "Old room name"
            },
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "state_key": "",
            "type": "m.room.name"
          }
        ],
        "end": "t47409-4357353_219380_26003_2269",
        "start": "t392-516_47314_0_7_1_1_1_11444_1"
    })
});

pub static SYNC_ROOM_MESSAGES_BATCH_2: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "chunk": [
          {
            "age": 1042,
            "content": {
              "body": "hello world",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Caaak:example.com",
            "origin_server_ts": 1444812213737i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@alice:example.com",
            "type": "m.room.message"
          },
          {
            "age": 20123,
            "content": {
              "body": "the world is big",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Cbbbk:example.com",
            "origin_server_ts": 1444812194656i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "type": "m.room.message"
          },
          {
            "age": 50789,
            "content": {
              "name": "New room name"
            },
            "event_id": "$1444812213350496Cccck:example.com",
            "origin_server_ts": 1444812163990i64,
            "prev_content": {
              "name": "Old room name"
            },
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "state_key": "",
            "type": "m.room.name"
          }
        ],
        "end": "t47409-4357353_219380_26003_2270",
        "start": "t47409-4357353_219380_26003_2269"
    })
});

pub static KEYS_QUERY: Lazy<JsonValue> = Lazy::new(|| {
    json!({
      "device_keys": {
        "@alice:example.org": {
          "JLAFKJWSCS": {
              "algorithms": [
                  "m.olm.v1.curve25519-aes-sha2",
                  "m.megolm.v1.aes-sha2"
              ],
              "device_id": "JLAFKJWSCS",
              "user_id": "@alice:example.org",
              "keys": {
                  "curve25519:JLAFKJWSCS": "wjLpTLRqbqBzLs63aYaEv2Boi6cFEbbM/sSRQ2oAKk4",
                  "ed25519:JLAFKJWSCS": "nE6W2fCblxDcOFmeEtCHNl8/l8bXcu7GKyAswA4r3mM"
              },
              "signatures": {
                  "@alice:example.org": {
                      "ed25519:JLAFKJWSCS": "m53Wkbh2HXkc3vFApZvCrfXcX3AI51GsDHustMhKwlv3TuOJMj4wistcOTM8q2+e/Ro7rWFUb9ZfnNbwptSUBA"
                  }
              },
              "unsigned": {
                  "device_display_name": "Alice's mobile phone"
              }
          }
        }
      },
      "failures": {}
    })
});

pub static KEYS_UPLOAD: Lazy<JsonValue> = Lazy::new(|| {
    json!({
      "one_time_key_counts": {
        "curve25519": 10,
        "signed_curve25519": 20
      }
    })
});

pub static LOGIN: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "access_token": "abc123",
        "device_id": "GHTYAJCE",
        "home_server": "matrix.org",
        "user_id": "@cheeky_monkey:matrix.org"
    })
});

pub static LOGIN_WITH_DISCOVERY: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "access_token": "abc123",
        "device_id": "GHTYAJCE",
        "home_server": "matrix.org",
        "user_id": "@cheeky_monkey:matrix.org",
        "well_known": {
            "m.homeserver": {
                "base_url": "https://example.org"
            },
            "m.identity_server": {
                "base_url": "https://id.example.org"
            }
        }
    })
});

pub static LOGIN_RESPONSE_ERR: Lazy<JsonValue> = Lazy::new(|| {
    json!({
      "errcode": "M_FORBIDDEN",
      "error": "Invalid password"
    })
});

pub static LOGIN_TYPES: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "flows": [
            {
                "type": "m.login.password"
            },
            {
                "type": "m.login.sso"
            },
            {
                "type": "m.login.token"
            }
        ]
    })
});

pub static EMPTY: Lazy<JsonValue> = Lazy::new(|| json!({}));

pub static EVENT_ID: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "event_id": "$h29iv0s8:example.com"
    })
});

pub static ENCRYPTION: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "algorithm": "m.megolm.v1.aes-sha2",
            "rotation_period_ms": 604800000,
            "rotation_period_msgs": 100
        },
        "event_id": "$143273582443PhrSn:example.org",
        "origin_server_ts": 1432735824653u64,
        "room_id": "!jEsUZKDJdhlrceRyVU:example.org",
        "sender": "@example:example.org",
        "state_key": "",
        "type": "m.room.encryption",
        "unsigned": {
            "age": 1234
        }
    })
});

// TODO: Move `prev_content` into `unsigned` once ruma supports it
pub static MEMBER: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "join"
        },
        "event_id": "$151800140517rfvjc:localhost",
        "membership": "join",
        "origin_server_ts": 151800140,
        "sender": "@example:localhost",
        "state_key": "@example:localhost",
        "type": "m.room.member",
        "prev_content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "invite"
        },
        "unsigned": {
            "age": 297036,
            "replaces_state": "$151800111315tsynI:localhost"
        }
    })
});

pub static MEMBER_INVITE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "avatar_url": "mxc://localhost/SEsfnsuifSDFSSEF",
            "displayname": "example",
            "membership": "invite",
            "reason": "Looking for support"
        },
        "event_id": "$143273582443PhrSn:localhost",
        "origin_server_ts": 1432735824,
        "room_id": "!jEsUZKDJdhlrceRyVU:localhost",
        "sender": "@example:localhost",
        "state_key": "@invited:localhost",
        "type": "m.room.member",
        "unsigned": {
            "age": 1234,
            "invite_room_state": [
                {
                    "content": {
                        "name": "Example Room"
                    },
                    "sender": "@example:localhost",
                    "state_key": "",
                    "type": "m.room.name"
                },
                {
                    "content": {
                        "join_rule": "invite"
                    },
                    "sender": "@example:localhost",
                    "state_key": "",
                    "type": "m.room.join_rules"
                }
            ]
        }
    })
});

// TODO: Move `prev_content` into `unsigned` once ruma supports it
pub static MEMBER_NAME_CHANGE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "avatar_url": null,
            "displayname": "changed",
            "membership": "join"
        },
        "event_id": "$151800234427abgho:localhost",
        "membership": "join",
        "origin_server_ts": 151800152,
        "sender": "@example:localhost",
        "state_key": "@example:localhost",
        "type": "m.room.member",
        "prev_content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "join"
        },
        "unsigned": {
            "age": 297032,
            "replaces_state": "$151800140517rfvjc:localhost"
        }
    })
});

pub static MEMBER_STRIPPED: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "join"
        },
        "sender": "@example:localhost",
        "state_key": "@example:localhost",
        "type": "m.room.member",
    })
});

pub static MESSAGE_EDIT: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "body": " * edited message",
            "m.new_content": {
                "body": "edited message",
                "msgtype": "m.text"
            },
            "m.relates_to": {
                "event_id": "$someeventid:foo",
                "rel_type": "m.replace"
            },
            "msgtype": "m.text"
        },
        "event_id": "$eventid:foo",
        "origin_server_ts": 159026265,
        "sender": "@alice:matrix.org",
        "type": "m.room.message",
        "unsigned": {
            "age": 85
        }
    })
});

pub static MESSAGE_EMOTE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "body": "is dancing", "format": "org.matrix.custom.html",
            "formatted_body": "<strong>is dancing</strong>",
            "msgtype": "m.emote"
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 152037280,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 598971
        }
    })
});

pub static MESSAGE_NOTICE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
      "origin_server_ts": 153356516,
      "sender": "@_neb_github:matrix.org",
      "event_id": "$153356516319138IHRIC:matrix.org",
      "unsigned": {
        "age": 743
      },
      "content": {
        "body": "https://github.com/matrix-org/matrix-python-sdk/issues/266 : Consider allowing MatrixClient.__init__ to take sync_token kwarg",
        "format": "org.matrix.custom.html",
        "formatted_body": "<a href='https://github.com/matrix-org/matrix-python-sdk/pull/313'>313: nio wins!</a>",
        "msgtype": "m.notice"
      },
      "type": "m.room.message",
      "room_id": "!YHhmBTmGBHGQOlGpaZ:matrix.org"
    })
});

pub static MESSAGE_TEXT: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "body": "is dancing", "format": "org.matrix.custom.html",
            "formatted_body": "<strong>is dancing</strong>",
            "msgtype": "m.text"
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 152037280,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 598971
        }
    })
});

pub static NAME: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "name": "room name"
        },
        "event_id": "$15139375513VdeRF:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.name",
        "unsigned": {
            "age": 703422
        }
    })
});

pub static NAME_STRIPPED: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "name": "room name"
        },
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.name",
    })
});

pub static POWER_LEVELS: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "ban": 50,
            "events": {
                "m.room.avatar": 50,
                "m.room.canonical_alias": 50,
                "m.room.history_visibility": 100,
                "m.room.name": 50,
                "m.room.power_levels": 100,
                "m.room.message": 25
            },
            "events_default": 0,
            "invite": 0,
            "kick": 50,
            "redact": 50,
            "state_default": 50,
            "users": {
                "@example:localhost": 100,
                "@bob:localhost": 0
            },
            "users_default": 0
        },
        "event_id": "$15139375512JaHAW:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.power_levels",
        "unsigned": {
            "age": 703422
        }
    })
});

pub static PRESENCE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "avatar_url": "mxc://localhost/wefuiwegh8742w",
            "currently_active": false,
            "last_active_ago": 1,
            "presence": "online",
            "status_msg": "Making cupcakes"
        },
        "sender": "@example:localhost",
        "type": "m.presence"
    })
});

pub static PUBLIC_ROOMS: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "chunk": [
            {
                "aliases": [
                    "#murrays:cheese.bar"
                ],
                "avatar_url": "mxc://bleeker.street/CHEDDARandBRIE",
                "guest_can_join": false,
                "name": "CHEESE",
                "num_joined_members": 37,
                "room_id": "!ol19s:bleecker.street",
                "topic": "Tasty tasty cheese",
                "world_readable": true
            }
        ],
        "next_batch": "p190q",
        "prev_batch": "p1902",
        "total_room_count_estimate": 115
    })
});

pub static PUSH_RULES: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "global": {
                "content": [
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "pattern": "example",
                        "rule_id": ".m.rule.contains_user_name"
                    }
                ],
                "override": [
                    {
                        "actions": [
                            "dont_notify"
                        ],
                        "conditions": [],
                        "default": true,
                        "enabled": false,
                        "rule_id": ".m.rule.master"
                    },
                    {
                        "actions": [
                            "dont_notify"
                        ],
                        "conditions": [
                            {
                                "key": "content.msgtype",
                                "kind": "event_match",
                                "pattern": "m.notice"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.suppress_notices"
                    }
                ],
                "room": [],
                "sender": [],
                "underride": [
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "ring"
                            },
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.call.invite"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.call"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight"
                            }
                        ],
                        "conditions": [
                            {
                                "kind": "contains_display_name"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.contains_display_name"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "is": "2",
                                "kind": "room_member_count"
                            },
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.message"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.room_one_to_one"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.member"
                            },
                            {
                                "key": "content.membership",
                                "kind": "event_match",
                                "pattern": "invite"
                            },
                            {
                                "key": "state_key",
                                "kind": "event_match",
                                "pattern": "@example:localhost"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.invite_for_me"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.member"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.member_event"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.message"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.message"
                    }
                ]
            }
        },
        "type": "m.push_rules"
    })
});

pub static REGISTRATION_RESPONSE_ERR: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "errcode": "M_FORBIDDEN",
        "error": "Invalid password",
        "completed": ["example.type.foo"],
        "flows": [
            {
                "stages": ["example.type.foo", "example.type.bar"]
            },
            {
                "stages": ["example.type.foo", "example.type.baz"]
            }
        ],
        "params": {
            "example.type.baz": {
                "example_key": "foobar"
            }
        },
        "session": "xxxxxx"
    })
});

pub static REACTION: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "m.relates_to": {
                "event_id": "$MDitXXXXXXuBlpP7S6c6XXXXXXXC2HqZ3peV1NrV4PKA",
                "key": "👍",
                "rel_type": "m.annotation"
            }
        },
        "event_id": "$QZn9xEXXXXXfd2tAGFH-XXgsffZlVMobk47Tl5Lpdtg",
        "origin_server_ts": 159027581,
        "sender": "@devinr528:matrix.org",
        "type": "m.reaction",
        "unsigned": {
            "age": 85
        }
    })
});

pub static READ_RECEIPT: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "$example": {
                "m.read": {
                    "@example:localhost": {
                        "ts": 1436451550
                    }
                }
            }
        },
        "room_id": "!test:localhost",
        "type": "m.receipt"
    })
});

pub static READ_RECEIPT_OTHER: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "$other": {
                "m.read": {
                    "@example:localhost": {
                        "ts": 1436964550
                    }
                }
            }
        },
        "room_id": "!test:localhost",
        "type": "m.receipt"
    })
});

pub static REDACTED_INVALID: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {},
        "event_id": "$15275046980maRLj:localhost",
        "origin_server_ts": 1527504698,
        "sender": "@example:localhost",
        "type": "m.room.message"
    })
});

pub static REDACTED_STATE: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {},
        "event_id": "$example_id:example.org",
        "origin_server_ts": 153232493,
        "sender": "@example:example.org",
        "state_key": "test_state_key",
        "type": "m.some.state",
        "unsigned": {
            "age": 3069315,
            "redacted_because": {
                "content": {},
                "event_id": "$redaction_example_id:example.org",
                "origin_server_ts": 153232494,
                "redacts": "$example_id:example.org",
                "sender": "@example:example:org",
                "type": "m.room.redaction",
                "unsigned": {"age": 30693147}
            },
            "redacted_by": "$redaction_example_id:example.org"
        }
    })
});

pub static REDACTED: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {},
        "event_id": "$15275046980maRLj:localhost",
        "origin_server_ts": 1527504698,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 19334,
            "redacted_because": {
                "content": {},
                "event_id": "$15275047031IXQRi:localhost",
                "origin_server_ts": 1527504703,
                "redacts": "$15275046980maRLj:localhost",
                "sender": "@example:localhost",
                "type": "m.room.redaction",
                "unsigned": {
                    "age": 14523
                }
            },
            "redacted_by": "$15275047031IXQRi:localhost"
        }
    })
});

pub static REDACTION: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "reason": "😀"
        },
        "event_id": "$151957878228ssqrJ:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "type": "m.room.redaction",
        "redacts": "$151957878228ssqrj:localhost"
    })
});

pub static ROOM_AVATAR: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "info": {
                "h": 398,
                "mimetype": "image/jpeg",
                "size": 31037,
                "w": 394
            },
            "url": "mxc://domain.com/JWEIFJgwEIhweiWJE"
        },
        "event_id": "$143273582443PhrSn:domain.com",
        "origin_server_ts": 143273582,
        "room_id": "!jEsUZKDJdhlrceRyVU:domain.com",
        "sender": "@example:domain.com",
        "state_key": "",
        "type": "m.room.avatar",
        "unsigned": {
            "age": 1234
        }
    })
});

pub static ROOM_ID: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "room_id": "!testroom:example.org"
    })
});

pub static TAG: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "tags": {
                "u.work": {
                    "order": 0.9
                }
            }
        },
        "type": "m.tag"
    })
});

// TODO: Move `prev_content` into `unsigned` once ruma supports it
pub static TOPIC: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "topic": "😀"
        },
        "event_id": "$151957878228ssqrJ:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.topic",
        "prev_content": {
            "topic": "test"
        },
        "unsigned": {
          "age": 1392989,
          "prev_sender": "@example:localhost",
          "replaces_state": "$151957069225EVYKm:localhost"
        }
    })
});

pub static TYPING: Lazy<JsonValue> = Lazy::new(|| {
    json!({
        "content": {
            "user_ids": [
                "@alice:matrix.org",
                "@bob:example.com"
            ]
        },
        "room_id": "!jEsUZKDJdhlrceRyVU:example.org",
        "type": "m.typing"
    })
});
