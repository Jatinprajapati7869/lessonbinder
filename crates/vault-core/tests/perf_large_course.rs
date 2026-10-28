//! Large-course list/search budgets — see `docs/PERF.md`.

use std::time::Instant;

use vault_core::course::course_detail;
use vault_core::library::{list_courses, CourseLibraryVisibility};
use vault_core::search::{search, SearchMatchKind};
use vault_core::VaultDb;

const LESSON_COUNT: usize = 500;

/// Keep in sync with `docs/PERF.md` (recorded on the reference machine).
const LIST_COURSES_BUDGET_MS: u64 = 250;
const COURSE_DETAIL_BUDGET_MS: u64 = 250;
const SEARCH_BUDGET_MS: u64 = 150;

fn seed_course_with_many_lessons(db: &VaultDb) -> String {
    let course_id = db
        .insert_course("Perf scale course", "/courses/perf-scale")
        .expect("insert course");
    for index in 0..LESSON_COUNT {
        let title = format!("Lesson {:04} marker-{:04}", index, index);
        let path = format!("videos/lesson-{:04}.mp4", index);
        db.insert_local_file_lesson(&course_id, &path, &title, index as i32)
            .expect("insert lesson");
    }
    course_id
}

#[test]
fn five_hundred_lesson_course_list_and_search_within_budget() {
    let db = VaultDb::open_in_memory().expect("open db");
    let course_id = seed_course_with_many_lessons(&db);

    let list_start = Instant::now();
    let courses = list_courses(&db, None, CourseLibraryVisibility::Active).expect("list courses");
    let list_courses_ms = list_start.elapsed().as_millis() as u64;
    assert_eq!(courses.len(), 1);
    assert_eq!(courses[0].course_id, course_id);

    let detail_start = Instant::now();
    let detail = course_detail(&db, &course_id).expect("course detail");
    let course_detail_ms = detail_start.elapsed().as_millis() as u64;
    assert_eq!(detail.lessons.len(), LESSON_COUNT);

    let search_start = Instant::now();
    let hits = search(&db, "marker-0250").expect("search");
    let search_ms = search_start.elapsed().as_millis() as u64;
    let title_hits = hits
        .iter()
        .filter(|hit| hit.match_kind == SearchMatchKind::LessonTitle)
        .count();
    assert_eq!(title_hits, 1);

    eprintln!(
        "perf_large_course: list_courses={list_courses_ms}ms course_detail={course_detail_ms}ms search={search_ms}ms"
    );

    assert!(
        list_courses_ms <= LIST_COURSES_BUDGET_MS,
        "list_courses took {list_courses_ms}ms (budget {LIST_COURSES_BUDGET_MS}ms)"
    );
    assert!(
        course_detail_ms <= COURSE_DETAIL_BUDGET_MS,
        "course_detail took {course_detail_ms}ms (budget {COURSE_DETAIL_BUDGET_MS}ms)"
    );
    assert!(
        search_ms <= SEARCH_BUDGET_MS,
        "search took {search_ms}ms (budget {SEARCH_BUDGET_MS}ms)"
    );
}
