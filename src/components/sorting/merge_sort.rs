use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use leptos::reactive::spawn_local;
use log::info;

fn merge_sort_stepwise(arr: &mut [i32], snapshots: &mut Vec<Vec<i32>>) {
    // info!("merge_sort_stepwise called with: {:?}", arr);

    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort_stepwise(&mut left, snapshots);
    merge_sort_stepwise(&mut right, snapshots);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        snapshots.push(arr.to_vec());
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
        snapshots.push(arr.to_vec());
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
        snapshots.push(arr.to_vec());
    }
}


#[component]
pub fn MergeSort() -> impl IntoView {
    let (sorting, set_sorting) = signal(false);
    let (speed, set_speed) = signal(200);
    let (current_step, set_current_step) = signal(0);
    let (snapshots, set_snapshots) = signal(vec![vec![50, 20, 35, 10, 40, 15]]);

    //Monitory updates
    Effect::watch(
        move || current_step.get(),
        move |step, _, _| {
            snapshots.with_untracked(|snaps| {
                if let Some(snapshot) = snaps.get(*step) {
                    info!("Step {}: {:?}", step, snapshot);
                } else {
                    info!("Step {}: <out of bounds>", step);
                }
            });
        },
        false,
    );

    // Trigger animation when sorting starts
    Effect::new(move |_| {
        if sorting.get() {
            info!("Sorting Effect triggered");

            spawn_local({
                let set_snapshots = set_snapshots.clone();
                let set_current_step = set_current_step.clone();
                let set_sorting = set_sorting.clone();
                let speed = speed.get(); // speed is a signal; get its value now

                async move {
                    let initial_data = snapshots.get().first().cloned().unwrap_or_default();
                    let mut data = initial_data.clone();
                    let mut steps = vec![];

                    merge_sort_stepwise(&mut data, &mut steps);
                    let total_steps = steps.len();

                    info!("Total steps: {}", total_steps);

                    set_snapshots.set(steps.clone());
                    set_current_step.set(0);

                    for (i, _snapshot) in steps.into_iter().enumerate() {
                        set_current_step.set(i);
                        TimeoutFuture::new(speed).await; // Use `speed` in ms
                    }

                    set_sorting.set(false);
                }
            });
        }
    });


    view! {
      <div class="container py-4 text-center">
            <h1 class="display-5 fw-bold mb-4">"Merge Sort Visualizer"</h1>
           <ControlPanel
                sorting
                set_sorting
                speed
                set_speed
                set_current_step
                set_snapshots
            />
            <Visualiser
                current_step
                snapshots
            />
        </div>
    }
}

#[component]
pub fn ControlPanel(
    sorting: ReadSignal<bool>,
    set_sorting: WriteSignal<bool>,
    speed: ReadSignal<u32>,
    set_speed: WriteSignal<u32>,
    set_current_step: WriteSignal<usize>,
    set_snapshots: WriteSignal<Vec<Vec<i32>>>,
) -> impl IntoView {

    view! {
         <div class="d-flex justify-content-center align-items-center gap-3 mb-4">
            <button class="btn btn-primary"
                on:click=move |_| set_sorting.set(true)
                disabled=move || sorting.get()
                >
                "Start"
            </button>
            <button class="btn btn-danger" on:click=move |_| {
                set_sorting.set(false);
                set_current_step.set(0);
                set_snapshots.set(vec![vec![50, 20, 35, 10, 40, 15]]); // or randomized
            }>
                "Reset"
            </button>
            <div>
                <label for="speedRange" class="form-label">"Speed (ms)"</label>
                <input
                    id="speedRange"
                    type="range"
                    min="10"
                    max="1000"
                    step="10"
                    class="form-range"
                    prop:value=move || speed.get()
                    on:input=move |ev| {
                        set_speed.set(event_target_value(&ev).parse::<u32>().unwrap_or(200));
                    }
                />
            </div>
        </div>
    }
}


#[component]
pub fn Visualiser(
    current_step: ReadSignal<usize>,
    snapshots: ReadSignal<Vec<Vec<i32>>>,
) -> impl IntoView {

    let current_array = Memo::new(move |_| {
        let step = current_step.get();
        snapshots.with(|snaps| snaps.get(step).cloned().unwrap_or_default())
    });
    info!("Visualiser::Current array: {:?}", current_array.get_untracked());


    view! {
        <div class="d-flex justify-content-center align-items-end gap-1" style="height: 300px;">
            <For
                each=move || current_array.get()
                key=|val| *val
                children=move |value| {
                    let height = format!("{}%", (value as f32) * 2.0); // scale height
                    view! {
                        <div
                            class="bg-primary mx-1"
                            style=move || format!("width: 20px; height: {height}; transition: height 0.3s;")
                        />
                    }
                }
            />
        </div>
    }
}
