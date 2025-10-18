use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps, CalendarNavigationProps,
    CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
};
use time::{macros::date, Date, UtcDateTime};
//use chrono::{Date, Utc, DateTime};

const CSS: Asset = asset!("/assets/styling/calendar.css");

#[component]
pub fn Calendar() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());

    rsx! {
        CalendarRaw {
            selected_date: selected_date(),
            on_date_change: move |date| {
                tracing::info!("Selected date: {:?}", date);
                selected_date.set(date);
            },
            view_date: view_date(),
            on_view_change: move |new_view: Date| {
                tracing::info!("View changed to: {}-{}", new_view.year(), new_view.month());
                view_date.set(new_view);
            },
            min_date: date!(1995 - 07 - 21),
            max_date: date!(2035 - 09 - 11),
            CalendarHeader {
                CalendarNavigation {
                    CalendarPreviousMonthButton {}
                    CalendarSelectMonth {}
                    CalendarSelectYear {}
                    CalendarNextMonthButton {}
                }
            }
            CalendarGrid {}
        }
    }
}

#[component]
pub fn CalendarRaw(props: CalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        div { class: "calendar",
            calendar::Calendar {
                selected_date: props.selected_date,
                on_date_change: props.on_date_change,
                on_format_weekday: props.on_format_weekday,
                on_format_month: props.on_format_month,
                view_date: props.view_date,
                today: props.today,
                on_view_change: props.on_view_change,
                disabled: props.disabled,
                first_day_of_week: props.first_day_of_week,
                min_date: props.min_date,
                max_date: props.max_date,
                attributes: props.attributes,
                {props.children}
            }
        }
    }
}

#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    rsx! {
        calendar::CalendarHeader { id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        calendar::CalendarNavigation { attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { attributes,
            svg {
                class: "calendar-previous-month-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "15 6 9 12 15 18" }
            }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { attributes,
            svg {
                class: "calendar-next-month-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "9 18 15 12 9 6" }
            }
        }
    }
}

#[component]
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth { class: "calendar-month-select", attributes: props.attributes }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear { class: "calendar-year-select", attributes: props.attributes }
    }
}

#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    rsx! {
        calendar::CalendarGrid {
            id: props.id,
            show_week_numbers: props.show_week_numbers,
            render_day: props.render_day,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    calendar::CalendarMonthTitle(props)
}
