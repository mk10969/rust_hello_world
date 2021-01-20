use std::{
    sync::{mpsc, Arc, Mutex, RwLock},
    thread::spawn,
};

// Iterator　trait 継承
pub trait OffThreadExt: Iterator {
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    // ここで、Sendを制約をかけているのがポイント
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);

        spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });
        receiver.into_iter()
    }
}

// thread 排他ロック
type PlayerId = i32;
const GAME_SIZE: usize = 8;
type WaitingList = Vec<PlayerId>;

// globalのスコープの場合、lazy_static!マクロを使う。
lazy_static! {
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}

struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>,
}

#[test]
fn test_mutex() {
    // Box, Arcを使って、ヒープ上に確保
    let app = Arc::new(FernEmpireApp {
        // Mutexを使って、データをロックする。
        waiting_list: Mutex::new(vec![]),
    });
}

impl FernEmpireApp {
    fn join_waiting_list(&self, player: PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();
        // deadlock
        // let mut guard = self.waiting_list.lock().unwrap();

        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            self.start_game(players);
        }
    }
    // Empty
    fn start_game(&self, players: Vec<PlayerId>) {}
}
