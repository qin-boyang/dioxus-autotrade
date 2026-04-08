use std::thread;
use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub fn play_beep() {
    // 启动新线程，避免阻塞交易逻辑
    thread::spawn(|| {
        // 1. 获取默认音频输出设备
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(e) => {
                println!("❌ 无法获取音频输出设备: {}", e);
                return;
            }
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(e) => {
                println!("❌ 无法创建音频 Sink: {}", e);
                return;
            }
        };

        // 2. 生成正弦波音源
        let source = SineWave::new(800.0) // 800Hz 是比较刺耳的报警频率
            .take_duration(Duration::from_secs(5)) // 修改：持续时间改为 5 秒
            .amplify(1.0); // 修改：1.0 为最大标准音量（不建议超过 1.0 否则会爆音/失真）

        // 3. 播放
        sink.append(source);

        // 4. 阻塞该线程直到播放结束（否则线程退出声音就没了）
        sink.sleep_until_end();
    });
}