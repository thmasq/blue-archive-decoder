use crate::blue_archive_generated::global::*;
use flatbuffers::{ForwardsUOffset, Vector};
use regex::Regex;
use std::collections::BTreeMap;

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

/// Generic migration function called by the macro
fn migrate_table_generic<T>(
    tx: &Transaction,
    source_table_name: &str,
    target_table_name: &str,
    parse_root_fn: fn(&[u8]) -> Result<T, flatbuffers::InvalidFlatbuffer>,
    get_data_list_fn: fn(
        T,
    ) -> Option<
        Vector<'static, ForwardsUOffset<impl flatbuffers::Follow<'static>>>,
    >,
    // Note: The above signature is pseudo-code for the closure logic used below in the macro
) -> Result<(), Box<dyn std::error::Error>> {
    // This function is implemented effectively inside the macro to handle the specific types
    Ok(())
}

macro_rules! migrate_table {
    ($tx:expr, $base_name:ident) => {
        paste::paste! {
            {
                let source_table = stringify!([< $base_name DBSchema >]);
                let target_table = stringify!([< $base_name Excel >]);

                println!("Migrating {} -> {}...", source_table, target_table);

                // 1. Get the bytes from the source database
                // Assuming the source table has a single row with the blob, or we aggregate them.
                // Usually in BA DBs, the table has columns, but the 'bytes' column in one of the rows (or the only row) contains the full flatbuffer list.
                let mut stmt = $tx.prepare(&format!("SELECT bytes FROM {} LIMIT 1", source_table))?;
                let bytes: Vec<u8> = stmt.query_row([], |row| row.get(0)).unwrap_or_default();

                if !bytes.is_empty() {
                    // 2. Decode the Flatbuffer
                    // root type is usually NameExcelTable
                    type RootTable<'a> = [< $base_name Excel >]<'a>;

                    match flatbuffers::root::<RootTable>(&bytes) {
                        Ok(root) => {
                            if let Some(list) = root.data_list() {
                                let mut first_row = true;

                                for item in list {
                                    // 3. Unpack to Native Struct (T) to get readable fields
                                    let native = item.unpack();

                                    // 4. Use Debug trait to inspect fields
                                    let debug_str = format!("{:?}", native);
                                    let (values_map, columns) = parse_debug_struct(&debug_str);

                                    // 5. Create Table (only once)
                                    if first_row {
                                        let col_defs: Vec<String> = columns.iter()
                                            .map(|(k, t)| format!("{} {}", k, t))
                                            .collect();
                                        let create_sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", target_table, col_defs.join(", "));
                                        $tx.execute(&create_sql, [])?;
                                        first_row = false;
                                    }

                                    // 6. Insert Row
                                    // We need to match the columns order defined in `columns`
                                    let col_names: Vec<&str> = columns.iter().map(|(k, _)| k.as_str()).collect();
                                    let vals: Vec<String> = columns.iter()
                                        .map(|(k, _)| values_map.get(k).cloned().unwrap_or("NULL".to_string()))
                                        .collect();

                                    let insert_sql = format!(
                                        "INSERT INTO {} ({}) VALUES ({})",
                                        target_table,
                                        col_names.join(", "),
                                        vals.join(", ")
                                    );

                                    $tx.execute(&insert_sql, [])?;
                                }
                            }
                        },
                        Err(e) => println!("  Error decoding Flatbuffer for {}: {:?}", source_table, e),
                    }
                } else {
                    println!("  No data found in {}", source_table);
                }
            }
        }
    };
}

pub fn migrate_all(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    let tx = conn.transaction()?;

    // List all tables here. The macro handles the naming convention:
    // Input: AccountLevel -> Reads AccountLevelDBSchema, Writes AccountLevelExcel

    migrate_table!(tx, AccountLevel);
    migrate_table!(tx, AssistEchelonTypeConvert);
    migrate_table!(tx, Attendance);
    migrate_table!(tx, AttendanceReward);
    migrate_table!(tx, AudioAnimator);
    migrate_table!(tx, BGM);
    migrate_table!(tx, BGMRaid);
    migrate_table!(tx, BGMUI);
    migrate_table!(tx, BGM_Global);
    migrate_table!(tx, Camera);
    migrate_table!(tx, CharacterDialog);
    migrate_table!(tx, CharacterDialogEmoji);
    migrate_table!(tx, CharacterDialogEvent);
    migrate_table!(tx, CharacterDialogSubtitle);
    migrate_table!(tx, CharacterGear);
    migrate_table!(tx, CharacterGearLevel);
    migrate_table!(tx, CharacterPotential);
    migrate_table!(tx, CharacterPotentialReward);
    migrate_table!(tx, CharacterPotentialStat);
    migrate_table!(tx, CharacterVoice);
    migrate_table!(tx, CharacterVoiceSubtitle);
    migrate_table!(tx, ClanAssistSlot);
    migrate_table!(tx, ClanChattingEmoji);
    migrate_table!(tx, ClanReward);
    migrate_table!(tx, CombatEmoji);
    migrate_table!(tx, ContentEnterCostReduce);
    migrate_table!(tx, ContentSpoilerPopup);
    migrate_table!(tx, ContentsScenario);
    migrate_table!(tx, ContentsShortcut);
    migrate_table!(tx, Currency);
    migrate_table!(tx, Emblem);
    migrate_table!(tx, EventContentNotify);
    migrate_table!(tx, EventContentSpoilerPopup);
    migrate_table!(tx, EventContentTreasureCellReward);
    migrate_table!(tx, EventContentTreasure);
    migrate_table!(tx, EventContentTreasureReward);
    migrate_table!(tx, EventContentTreasureRound);
    migrate_table!(tx, FarmingDungeonLocationManage);
    migrate_table!(tx, FavorLevel);
    migrate_table!(tx, FavorLevelReward);
    migrate_table!(tx, FixedEchelonSetting);
    migrate_table!(tx, FormationLocation);
    migrate_table!(tx, Ground);
    migrate_table!(tx, GroundModuleReward);
    migrate_table!(tx, IdCardBackground);
    migrate_table!(tx, Information);
    migrate_table!(tx, LoadingImage);
    migrate_table!(tx, LocalizeCharProfileChange);
    migrate_table!(tx, Localize);
    migrate_table!(tx, LocalizeError);
    migrate_table!(tx, LocalizeEtc);
    migrate_table!(tx, LocalizeSkill);
    migrate_table!(tx, MemoryLobby);
    migrate_table!(tx, MemoryLobby_Global);
    migrate_table!(tx, MessagePopup);
    migrate_table!(tx, MiniGameDefenseCharacterBan);
    migrate_table!(tx, MiniGameDefenseFixedStat);
    migrate_table!(tx, MiniGameDefenseInfo);
    migrate_table!(tx, MiniGameDefenseStage);
    migrate_table!(tx, MiniGameDreamCollectionScenario);
    migrate_table!(tx, MiniGameDreamDailyPoint);
    migrate_table!(tx, MiniGameDreamEnding);
    migrate_table!(tx, MiniGameDreamEndingReward);
    migrate_table!(tx, MiniGameDreamInfo);
    migrate_table!(tx, MiniGameDreamParameter);
    migrate_table!(tx, MiniGameDreamReplayScenario);
    migrate_table!(tx, MiniGameDreamSchedule);
    migrate_table!(tx, MiniGameDreamScheduleResult);
    migrate_table!(tx, MiniGameDreamTimeline);
    migrate_table!(tx, MiniGameRoadPuzzleInfo);
    migrate_table!(tx, MiniGameRoadPuzzleRailSetReward);
    migrate_table!(tx, MiniGameRoadPuzzleReward);
    migrate_table!(tx, MiniGameRoadPuzzleVoice);
    migrate_table!(tx, MinigameDreamVoice);
    migrate_table!(tx, MinigameRoadPuzzleAdditionalReward);
    migrate_table!(tx, MinigameRoadPuzzleMap);
    migrate_table!(tx, MinigameRoadPuzzleMapTile);
    migrate_table!(tx, MinigameRoadPuzzleRailTile);
    migrate_table!(tx, MinigameRoadPuzzleRoadRound);
    migrate_table!(tx, MissionEmergencyComplete);
    migrate_table!(tx, MultiFloorRaidReward);
    migrate_table!(tx, MultiFloorRaidSeasonManage);
    migrate_table!(tx, MultiFloorRaidStage);
    migrate_table!(tx, MultiFloorRaidStatChange);
    migrate_table!(tx, ObstacleStat);
    migrate_table!(tx, OpenCondition);
    migrate_table!(tx, Operator);
    migrate_table!(tx, ScenarioBGEffect);
    migrate_table!(tx, ScenarioBGName);
    migrate_table!(tx, ScenarioBGName_Global);
    migrate_table!(tx, ScenarioCharacterEmotion);
    migrate_table!(tx, ScenarioCharacterName);
    migrate_table!(tx, ScenarioCharacterSituationSet);
    migrate_table!(tx, ScenarioContentCollection);
    migrate_table!(tx, ScenarioEffect);
    migrate_table!(tx, ScenarioMode);
    migrate_table!(tx, ScenarioModeReward);
    migrate_table!(tx, ScenarioResourceInfo);
    migrate_table!(tx, ScenarioScript);
    migrate_table!(tx, ScenarioTransition);
    migrate_table!(tx, SchoolDungeonReward);
    migrate_table!(tx, SchoolDungeonStage);
    migrate_table!(tx, ServiceAction);
    migrate_table!(tx, ShortcutType);
    migrate_table!(tx, SkillAdditionalTooltip);
    migrate_table!(tx, SoundUI);
    migrate_table!(tx, SpineLipsync);
    migrate_table!(tx, StageFileRefreshSetting);
    migrate_table!(tx, StatLevelInterpolation);
    migrate_table!(tx, StickerGroup);
    migrate_table!(tx, StickerPageContent);
    migrate_table!(tx, StoryStrategy);
    migrate_table!(tx, Toast);
    migrate_table!(tx, TutorialCharacterDialog);
    migrate_table!(tx, Tutorial);
    migrate_table!(tx, TutorialFailureImage);
    migrate_table!(tx, UnderCoverStage);
    migrate_table!(tx, Video);
    migrate_table!(tx, Video_Global);
    migrate_table!(tx, VoiceCommon);
    migrate_table!(tx, Voice);
    migrate_table!(tx, VoiceLogicEffect);
    migrate_table!(tx, VoiceRoomException);
    migrate_table!(tx, VoiceSpine);
    migrate_table!(tx, VoiceTimeline);
    migrate_table!(tx, WorldRaidCondition);

    tx.commit()?;
    Ok(())
}
