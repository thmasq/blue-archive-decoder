use crate::ba_flatbuffers::global::*;
use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::HashMap;
use std::io::Cursor;

// Define the signature for a loader function.
// It takes a reference to the database and returns a tuple of (Column Names, Rows).
pub type TableLoader = Box<
    dyn Fn(
        &mut Database<Cursor<Vec<u8>>>,
    ) -> Result<(Vec<String>, Vec<Vec<Value>>), Box<dyn std::error::Error>>,
>;

/// Converts a serde_json::Value to a sqlite_wasm_reader::Value
fn json_to_sqlite_value(val: &serde_json::Value) -> Value {
    match val {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Integer(if *b { 1 } else { 0 }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Real(f)
            } else {
                // Fallback for types that fit neither (should be rare in this context)
                Value::Text(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::Text(s.clone()),
        // Serialize Arrays and Objects back to JSON strings for storage/display
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => Value::Text(val.to_string()),
    }
}

macro_rules! register_table {
    // Pattern 1: Standard naming (recurses to Pattern 2)
    ($registry:expr, $base_name:ident) => {
        paste::paste! {
            register_table!($registry, $base_name, [< $base_name Excel >], [< $base_name ExcelT >])
        }
    };

    // Pattern 2: Explicit naming
    // Note: $native_type is unused here as we serialize the flatbuffer root ($root_type) directly
    ($registry:expr, $base_name:ident, $root_type:ident, $native_type:ident) => {
        paste::paste! {
            {
                let source_table = stringify!([< $base_name DBSchema >]);
                let target_table = stringify!([< $base_name Excel >]); // The name shown in the viewer

                // Create the loader closure
                let loader = Box::new(move |db: &mut Database<Cursor<Vec<u8>>>| {
                    // 1. Get the bytes from ALL rows
                    let query_str = format!("SELECT Bytes FROM {}", source_table);
                    let query = SelectQuery::parse(&query_str).map_err(|e| format!("Query parse error: {:?}", e))?;

                    let rows = db.execute_query(&query).map_err(|e| format!("DB exec error: {:?}", e))?;

                    let mut result_rows = Vec::new();
                    let mut headers = Vec::new();
                    let mut first = true;

                    // 2. Iterate over every row in the DB table
                    for row in rows {
                         let bytes = match row.values().next() {
                            Some(Value::Blob(b)) => b.clone(),
                            Some(Value::Text(s)) => s.as_bytes().to_vec(),
                            _ => continue, // Skip rows with no valid blob
                        };

                        if bytes.is_empty() {
                            continue;
                        }

                        // 3. Decode Flatbuffer Item directly
                        // We use the root type ($root_type / SomethingExcel) which implements Serialize
                        type RootItem<'a> = $root_type<'a>;

                        // Note: flatbuffers::root verifies the buffer.
                        let root = flatbuffers::root::<RootItem>(&bytes)?;

                        // 4. Serialize to JSON
                        // We rely on the flatbuffer struct implementing Serialize (via --rust-serialize)
                        let json_val = serde_json::to_value(&root)?;

                        if let serde_json::Value::Object(map) = json_val {
                            if first {
                                // Extract headers from the first row.
                                // serde_json::Map is backed by BTreeMap, so keys are sorted alphabetically.
                                headers = map.keys().cloned().collect();
                                first = false;
                            }

                            // Ensure values follow header order
                            let mut row_vec = Vec::new();
                            for header in &headers {
                                let val = map.get(header).unwrap_or(&serde_json::Value::Null);
                                row_vec.push(json_to_sqlite_value(val));
                            }

                            result_rows.push(row_vec);
                        }
                    }

                    Ok((headers, result_rows))
                });

                $registry.insert(target_table.to_string(), loader);
            }
        }
    };
}

pub fn register_loaders(registry: &mut HashMap<String, TableLoader>) {
    register_table!(registry, AccountLevel);
    register_table!(registry, AssistEchelonTypeConvert);
    register_table!(registry, Attendance);
    register_table!(registry, AttendanceReward);
    // register_table!(registry, AudioAnimator);
    register_table!(registry, BGM);
    register_table!(registry, BGMRaid);
    register_table!(registry, BGMUI);
    register_table!(registry, BGM_Global);
    register_table!(registry, Camera);
    register_table!(registry, CharacterDialog);
    register_table!(registry, CharacterDialogEmoji);
    register_table!(registry, CharacterDialogEvent);
    register_table!(registry, CharacterDialogSubtitle);
    register_table!(registry, CharacterGear);
    register_table!(registry, CharacterGearLevel);
    register_table!(registry, CharacterPotential);
    register_table!(registry, CharacterPotentialReward);
    register_table!(registry, CharacterPotentialStat);
    register_table!(registry, CharacterVoice);
    register_table!(registry, CharacterVoiceSubtitle);
    register_table!(registry, ClanAssistSlot);
    register_table!(registry, ClanChattingEmoji);
    register_table!(registry, ClanReward);
    register_table!(registry, CombatEmoji);
    register_table!(registry, ContentEnterCostReduce);
    register_table!(registry, ContentSpoilerPopup);
    register_table!(registry, ContentsScenario);
    register_table!(registry, ContentsShortcut);
    register_table!(registry, Currency);
    // register_table!(registry, Emblem);
    register_table!(registry, EventContentNotify);
    register_table!(registry, EventContentSpoilerPopup);
    register_table!(registry, EventContentTreasureCellReward);
    register_table!(registry, EventContentTreasure);
    register_table!(registry, EventContentTreasureReward);
    register_table!(registry, EventContentTreasureRound);
    register_table!(registry, FarmingDungeonLocationManage);
    register_table!(registry, FavorLevel);
    register_table!(registry, FavorLevelReward);
    register_table!(registry, FixedEchelonSetting);
    register_table!(registry, FormationLocation);
    // register_table!(registry, Ground);
    register_table!(registry, GroundModuleReward);
    register_table!(registry, IdCardBackground);
    register_table!(registry, Information);
    register_table!(registry, LoadingImage);
    register_table!(registry, LocalizeCharProfileChange);
    register_table!(registry, Localize);
    register_table!(registry, LocalizeError);
    register_table!(registry, LocalizeEtc);
    register_table!(registry, LocalizeSkill);
    register_table!(registry, MemoryLobby);
    register_table!(registry, MemoryLobby_Global);
    register_table!(registry, MessagePopup);
    register_table!(registry, MiniGameDefenseCharacterBan);
    register_table!(registry, MiniGameDefenseFixedStat);
    register_table!(registry, MiniGameDefenseInfo);
    register_table!(registry, MiniGameDefenseStage);
    register_table!(registry, MiniGameDreamCollectionScenario);
    register_table!(registry, MiniGameDreamDailyPoint);
    register_table!(registry, MiniGameDreamEnding);
    register_table!(registry, MiniGameDreamEndingReward);
    register_table!(registry, MiniGameDreamInfo);
    register_table!(registry, MiniGameDreamParameter);
    register_table!(registry, MiniGameDreamReplayScenario);
    register_table!(registry, MiniGameDreamSchedule);
    register_table!(registry, MiniGameDreamScheduleResult);
    register_table!(registry, MiniGameDreamTimeline);
    register_table!(registry, MiniGameRoadPuzzleInfo);
    register_table!(registry, MiniGameRoadPuzzleRailSetReward);
    register_table!(registry, MiniGameRoadPuzzleReward);
    register_table!(registry, MiniGameRoadPuzzleVoice);
    // register_table!(registry, MiniGameDreamVoice);
    // register_table!(registry, MiniGameRoadPuzzleAdditionalReward);
    register_table!(registry, MiniGameRoadPuzzleMap);
    // register_table!(registry, MiniGameRoadPuzzleMapTile);
    // register_table!(registry, MiniGameRoadPuzzleRailTile);
    // register_table!(registry, MiniGameRoadPuzzleRoadRound);
    register_table!(registry, MissionEmergencyComplete);
    register_table!(registry, MultiFloorRaidReward);
    register_table!(registry, MultiFloorRaidSeasonManage);
    register_table!(registry, MultiFloorRaidStage);
    register_table!(registry, MultiFloorRaidStatChange);
    register_table!(registry, ObstacleStat);
    register_table!(registry, OpenCondition);
    register_table!(registry, Operator);
    register_table!(registry, ScenarioBGEffect);
    register_table!(registry, ScenarioBGName);
    register_table!(registry, ScenarioBGName_Global);
    register_table!(registry, ScenarioCharacterEmotion);
    register_table!(registry, ScenarioCharacterName);
    register_table!(registry, ScenarioCharacterSituationSet);
    register_table!(registry, ScenarioContentCollection);
    register_table!(registry, ScenarioEffect);
    register_table!(registry, ScenarioMode);
    register_table!(registry, ScenarioModeReward);
    register_table!(registry, ScenarioResourceInfo);
    register_table!(registry, ScenarioScript);
    register_table!(registry, ScenarioTransition);
    register_table!(registry, SchoolDungeonReward);
    register_table!(registry, SchoolDungeonStage);
    register_table!(registry, ServiceAction);
    register_table!(registry, ShortcutType);
    register_table!(registry, SkillAdditionalTooltip);
    register_table!(registry, SoundUI);
    register_table!(registry, SpineLipsync);
    register_table!(registry, StageFileRefreshSetting);
    register_table!(registry, StatLevelInterpolation);
    register_table!(registry, StickerGroup);
    register_table!(registry, StickerPageContent);
    register_table!(registry, StoryStrategy);
    register_table!(registry, Toast);
    register_table!(registry, TutorialCharacterDialog);
    register_table!(registry, Tutorial);
    register_table!(registry, TutorialFailureImage);
    register_table!(registry, UnderCoverStage);
    register_table!(registry, Video);
    register_table!(registry, Video_Global);
    register_table!(registry, VoiceCommon);
    register_table!(registry, Voice);
    register_table!(registry, VoiceLogicEffect);
    register_table!(registry, VoiceRoomException);
    register_table!(registry, VoiceSpine);
    register_table!(registry, VoiceTimeline);
    register_table!(registry, WorldRaidCondition);
}
