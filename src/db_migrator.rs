use crate::blue_archive_generated::global::*;
use regex::Regex;
use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::BTreeMap;
use std::io::{Read, Seek};

/// Helper function to parse the Debug string of a struct and extract fields.
/// Returns a map of "field_name" -> "SQL_value_string" and a list of column definitions.
fn parse_debug_struct(debug_str: &str) -> (BTreeMap<String, String>, Vec<(String, String)>) {
    // Regex to find "key: value" pairs.
    // This is a simplified parser; complex nested structures (vectors/tables) will be treated as strings.
    let re = Regex::new(r"(\w+):\s*((?:\[.*?\])|(?:\{.*?\})|(?:[^,]+))").unwrap();

    let mut values = BTreeMap::new();
    let mut columns = Vec::new();

    for cap in re.captures_iter(debug_str) {
        let key = cap[1].to_string();
        let raw_val = cap[2].trim().to_string();

        // Infer SQL type and format value
        let (sql_type, sql_val) = if raw_val.starts_with('"') {
            ("TEXT", raw_val) // Already quoted
        } else if raw_val == "true" || raw_val == "false" {
            (
                "INTEGER",
                if raw_val == "true" {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            )
        } else if raw_val.chars().all(|c| c.is_digit(10) || c == '-') {
            ("INTEGER", raw_val)
        } else if raw_val.contains('.') && raw_val.parse::<f64>().is_ok() {
            ("REAL", raw_val)
        } else if raw_val.starts_with('[')
            || raw_val.starts_with('{')
            || raw_val.starts_with("Some")
        {
            // Arrays, sub-structs, or wrapped options -> Store as TEXT
            ("TEXT", format!("'{}'", raw_val.replace("'", "''")))
        } else {
            // Enums usually print their variant name in Debug, treat as TEXT
            ("TEXT", format!("'{}'", raw_val.replace("'", "''")))
        };

        values.insert(key.clone(), sql_val);
        columns.push((key, sql_type.to_string()));
    }

    (values, columns)
}

macro_rules! migrate_table {
    // Pattern 1: Standard naming
    // Recursively calls Pattern 2 with the default generated names
    ($db:expr, $base_name:ident) => {
        paste::paste! {
            migrate_table!($db, $base_name, [< $base_name ExcelTable >], [< $base_name ExcelT >])
        }
    };

    // Pattern 2: Explicit naming
    // Used when the Rust struct names don't strictly match the base name (e.g. BGM_Global -> BgmGlobal)
    ($db:expr, $base_name:ident, $root_type:ident, $native_type:ident) => {
        paste::paste! {
            {
                // The SQL table names usually strictly follow the base name string
                let source_table = stringify!([< $base_name DBSchema >]);
                let target_table = stringify!([< $base_name Excel >]);

                println!("-- Migrating {} -> {}...", source_table, target_table);

                // 1. Get the bytes from the source database using sqlite-wasm-reader
                let query_str = format!("SELECT bytes FROM {} LIMIT 1", source_table);
                let bytes: Vec<u8> = match SelectQuery::parse(&query_str) {
                    Ok(query) => {
                        match $db.execute_query(&query) {
                            Ok(rows) => {
                                if let Some(row) = rows.first() {
                                    // Try to get the blob from the first available value
                                    match row.values().next() {
                                        Some(Value::Blob(b)) => b.clone(),
                                        Some(Value::Text(s)) => s.as_bytes().to_vec(),
                                        _ => Vec::new(),
                                    }
                                } else {
                                    Vec::new()
                                }
                            }
                            Err(e) => {
                                eprintln!("-- Error executing query for {}: {:?}", source_table, e);
                                Vec::new()
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("-- Error parsing query for {}: {:?}", source_table, e);
                        Vec::new()
                    }
                };

                if !bytes.is_empty() {
                    // 2. Decode the Flatbuffer using the explicit Root Type
                    type RootTable<'a> = $root_type<'a>;

                    match flatbuffers::root::<RootTable>(&bytes) {
                        Ok(root) => {
                            if let Some(list) = root.data_list() {
                                let mut first_row = true;

                                for item in list {
                                    // 3. Unpack to Native Struct (T)
                                    // Use the explicit Native Type
                                    let native: $native_type = item.unpack();

                                    // 4. Use Debug trait to inspect fields
                                    let debug_str = format!("{:?}", native);
                                    let (values_map, columns) = parse_debug_struct(&debug_str);

                                    // 5. Create Table SQL (only once)
                                    if first_row {
                                        let col_defs: Vec<String> = columns.iter()
                                            .map(|(k, t)| format!("{} {}", k, t))
                                            .collect();

                                        println!("CREATE TABLE IF NOT EXISTS {} ({});", target_table, col_defs.join(", "));
                                        first_row = false;
                                    }

                                    // 6. Insert Row SQL
                                    let col_names: Vec<&str> = columns.iter().map(|(k, _)| k.as_str()).collect();
                                    let vals: Vec<String> = columns.iter()
                                        .map(|(k, _)| values_map.get(k).cloned().unwrap_or("NULL".to_string()))
                                        .collect();

                                    println!(
                                        "INSERT INTO {} ({}) VALUES ({});",
                                        target_table,
                                        col_names.join(", "),
                                        vals.join(", ")
                                    );
                                }
                            }
                        },
                        Err(e) => eprintln!("-- Error decoding Flatbuffer for {}: {:?}", source_table, e),
                    }
                } else {
                    println!("-- No data found in {}", source_table);
                }
            }
        }
    };
}

pub fn migrate_all<IO: Read + Seek>(
    db: &mut Database<IO>,
) -> Result<(), Box<dyn std::error::Error>> {
    migrate_table!(db, AccountLevel);
    migrate_table!(db, AssistEchelonTypeConvert);
    migrate_table!(db, Attendance);
    migrate_table!(db, AttendanceReward);
    migrate_table!(db, AudioAnimator);
    migrate_table!(db, BGM);
    migrate_table!(db, BGMRaid);
    migrate_table!(db, BGMUI);
    migrate_table!(db, BGM_Global, BgmGlobalExcelTable, BgmGlobalExcelT);
    migrate_table!(db, Camera);
    migrate_table!(db, CharacterDialog);
    migrate_table!(db, CharacterDialogEmoji);
    migrate_table!(db, CharacterDialogEvent);
    migrate_table!(db, CharacterDialogSubtitle);
    migrate_table!(db, CharacterGear);
    migrate_table!(db, CharacterGearLevel);
    migrate_table!(db, CharacterPotential);
    migrate_table!(db, CharacterPotentialReward);
    migrate_table!(db, CharacterPotentialStat);
    migrate_table!(db, CharacterVoice);
    migrate_table!(db, CharacterVoiceSubtitle);
    migrate_table!(db, ClanAssistSlot);
    migrate_table!(db, ClanChattingEmoji);
    migrate_table!(db, ClanReward);
    migrate_table!(db, CombatEmoji);
    migrate_table!(db, ContentEnterCostReduce);
    migrate_table!(db, ContentSpoilerPopup);
    migrate_table!(db, ContentsScenario);
    migrate_table!(db, ContentsShortcut);
    migrate_table!(db, Currency);
    migrate_table!(db, Emblem);
    migrate_table!(db, EventContentNotify);
    migrate_table!(db, EventContentSpoilerPopup);
    migrate_table!(db, EventContentTreasureCellReward);
    migrate_table!(db, EventContentTreasure);
    migrate_table!(db, EventContentTreasureReward);
    migrate_table!(db, EventContentTreasureRound);
    migrate_table!(db, FarmingDungeonLocationManage);
    migrate_table!(db, FavorLevel);
    migrate_table!(db, FavorLevelReward);
    migrate_table!(db, FixedEchelonSetting);
    migrate_table!(db, FormationLocation);
    migrate_table!(db, Ground);
    migrate_table!(db, GroundModuleReward);
    migrate_table!(db, IdCardBackground);
    migrate_table!(db, Information);
    migrate_table!(db, LoadingImage);
    migrate_table!(db, LocalizeCharProfileChange);
    migrate_table!(db, Localize);
    migrate_table!(db, LocalizeError);
    migrate_table!(db, LocalizeEtc);
    migrate_table!(db, LocalizeSkill);
    migrate_table!(db, MemoryLobby);
    migrate_table!(
        db,
        MemoryLobby_Global,
        MemoryLobbyGlobalExcelTable,
        MemoryLobbyGlobalExcelT
    );
    migrate_table!(db, MessagePopup);
    migrate_table!(db, MiniGameDefenseCharacterBan);
    migrate_table!(db, MiniGameDefenseFixedStat);
    migrate_table!(db, MiniGameDefenseInfo);
    migrate_table!(db, MiniGameDefenseStage);
    migrate_table!(db, MiniGameDreamCollectionScenario);
    migrate_table!(db, MiniGameDreamDailyPoint);
    migrate_table!(db, MiniGameDreamEnding);
    migrate_table!(db, MiniGameDreamEndingReward);
    migrate_table!(db, MiniGameDreamInfo);
    migrate_table!(db, MiniGameDreamParameter);
    migrate_table!(db, MiniGameDreamReplayScenario);
    migrate_table!(db, MiniGameDreamSchedule);
    migrate_table!(db, MiniGameDreamScheduleResult);
    migrate_table!(db, MiniGameDreamTimeline);
    migrate_table!(db, MiniGameRoadPuzzleInfo);
    migrate_table!(db, MiniGameRoadPuzzleRailSetReward);
    migrate_table!(db, MiniGameRoadPuzzleReward);
    migrate_table!(db, MiniGameRoadPuzzleVoice);
    migrate_table!(db, MinigameDreamVoice);
    migrate_table!(
        db,
        MinigameRoadPuzzleAdditionalReward,
        MiniGameRoadPuzzleAdditionalRewardExcelTable,
        MiniGameRoadPuzzleAdditionalRewardExcelT
    );
    migrate_table!(db, MinigameRoadPuzzleMap);
    migrate_table!(db, MinigameRoadPuzzleMapTile);
    migrate_table!(db, MinigameRoadPuzzleRailTile);
    migrate_table!(db, MinigameRoadPuzzleRoadRound);
    migrate_table!(
        db,
        MinigameRoadPuzzleAdditionalReward,
        MiniGameRoadPuzzleAdditionalRewardExcelTable,
        MiniGameRoadPuzzleAdditionalRewardExcelT
    );
    migrate_table!(db, MissionEmergencyComplete);
    migrate_table!(db, MultiFloorRaidReward);
    migrate_table!(db, MultiFloorRaidSeasonManage);
    migrate_table!(db, MultiFloorRaidStage);
    migrate_table!(db, MultiFloorRaidStatChange);
    migrate_table!(db, ObstacleStat);
    migrate_table!(db, OpenCondition);
    migrate_table!(db, Operator);
    migrate_table!(db, ScenarioBGEffect);
    migrate_table!(db, ScenarioBGName);
    migrate_table!(
        db,
        ScenarioBGName_Global,
        ScenarioBgnameGlobalExcelTable,
        ScenarioBgnameGlobalExcelT
    );
    migrate_table!(db, ScenarioCharacterEmotion);
    migrate_table!(db, ScenarioCharacterName);
    migrate_table!(db, ScenarioCharacterSituationSet);
    migrate_table!(db, ScenarioContentCollection);
    migrate_table!(db, ScenarioEffect);
    migrate_table!(db, ScenarioMode);
    migrate_table!(db, ScenarioModeReward);
    migrate_table!(db, ScenarioResourceInfo);
    migrate_table!(db, ScenarioScript);
    migrate_table!(db, ScenarioTransition);
    migrate_table!(db, SchoolDungeonReward);
    migrate_table!(db, SchoolDungeonStage);
    migrate_table!(db, ServiceAction);
    migrate_table!(db, ShortcutType);
    migrate_table!(db, SkillAdditionalTooltip);
    migrate_table!(db, SoundUI);
    migrate_table!(db, SpineLipsync);
    migrate_table!(db, StageFileRefreshSetting);
    migrate_table!(db, StatLevelInterpolation);
    migrate_table!(db, StickerGroup);
    migrate_table!(db, StickerPageContent);
    migrate_table!(db, StoryStrategy);
    migrate_table!(db, Toast);
    migrate_table!(db, TutorialCharacterDialog);
    migrate_table!(db, Tutorial);
    migrate_table!(db, TutorialFailureImage);
    migrate_table!(db, UnderCoverStage);
    migrate_table!(db, Video);
    migrate_table!(db, Video_Global, VideoGlobalExcelTable, VideoGlobalExcelT);
    migrate_table!(db, VoiceCommon);
    migrate_table!(db, Voice);
    migrate_table!(db, VoiceLogicEffect);
    migrate_table!(db, VoiceRoomException);
    migrate_table!(db, VoiceSpine);
    migrate_table!(db, VoiceTimeline);
    migrate_table!(db, WorldRaidCondition);

    Ok(())
}
