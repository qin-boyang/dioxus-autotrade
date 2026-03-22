use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavLayout() -> Element {
    rsx! {
        // 主容器：使用 bg-zinc-900 作为背景，text-zinc-100 作为主文字颜色
        div { class: "flex flex-col h-screen bg-zinc-900 text-zinc-100 overflow-hidden fixed inset-0",

            // 内容区域
            div { class: "flex-1 overflow-auto p-4",
                Outlet::<Route> {}
            }

            // 底部导航栏：深色背景 + 顶部细边框
            nav { class: "h-24 border-t border-zinc-800 bg-zinc-900/95 flex justify-around items-center px-4 pb-safe",

                 // 买入按钮：绿色（交易通常用绿买红卖）
                 Link {
                    to: Route::Buy {},
                    class: "flex flex-col items-center justify-center flex-1 h-12 mx-2
                            rounded-lg transition-all duration-150
                            bg-emerald-600 text-white hover:bg-emerald-500
                            active:scale-95 active:bg-emerald-700 shadow-lg",
                    "买入BTC界面"
                 }

                 // 卖出按钮：红色
                 Link {
                    to: Route::Sell {},
                    class: "flex flex-col items-center justify-center flex-1 h-12 mx-2
                            rounded-lg transition-all duration-150
                            bg-rose-600 text-white hover:bg-rose-500
                            active:scale-95 active:bg-rose-700 shadow-lg",
                    "卖出BTC界面"
                 }

                 // 设置按钮：深灰背景
                 Link {
                    to: Route::Settings {},
                    class: "flex flex-col items-center justify-center flex-1 h-12 mx-2
                            rounded-lg transition-all duration-150
                            bg-zinc-800 text-zinc-300 hover:bg-zinc-700
                            active:scale-95 border border-zinc-700",
                    "设置API界面"
                 }
            }
        }
    }
}