use std::thread;
use std::sync::mpsc::channel;
// use std::collections::VecDeque;
use std::time::Duration;
// use rand::Rng;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
// use std::thread::Thread;
// use std::cell::RefCell;
use rand::Rng;

pub fn main_channel_thread() {
    test();
}

#[derive(Debug)]
struct RtnMarketData(String);

#[derive(Debug)]
struct RtnOrder(String);

#[derive(Debug)]
struct RtnTrade(String);


#[derive(Debug)]
//模拟thost传过来的不同的值；
//数据类型 提供给管道判断
enum TradeData {
    //返回市场数据
    RtnMarketData(String),
    //返回排序
    RtnOrder(String),
    //返回交易
    RtnTrade(String),
}

//监听者类
struct Strategy {
    //策略
    stra_name: String,
    //线程
    stra_thread_builder: thread::Builder,
    //订阅的相关合约
    stra_instructions: Vec<String>,
}

//行动计划 监听者
impl Strategy {
    fn new(name: String, instrs: Vec<String>) -> Self {
        Self {
            stra_name: name,
            stra_thread_builder: thread::Builder::new(),
            stra_instructions: instrs,
        }
    }
    fn on_rtn_market_data(&self, md: &TradeData) {
        println!(" -> strategy:{:?} RtnMarketData=> recv:{:?}", self.stra_name, md);
    }
    fn on_rtn_order(&self, order: &TradeData) {
        println!(" -> strategy:{:?} RtnOrder => recv:{:?}", self.stra_name, order);
    }
    fn on_rtn_trade(&self, trade: &TradeData) {
        println!(" -> strategy:{:?} RtnTrade=> recv:{:?}", self.stra_name, trade);
    }
}


struct StrategyGroup {
    //stra_list : RefCell<Vec<Strategy>>,
    stra_list: Vec<Strategy>,
}

impl StrategyGroup {
    fn new(list: Vec<Strategy>) -> Self {
        //stra_list:vec![Strategy::new("DSCJ",vec!["IC","IF"]),Strategy::new("WSDJ",vec!["cu,ag"])]
        Self {
            //stra_list:RefCell::new(list),
            stra_list: list,
        }
    }
}

//监听者管理
struct StrategyManager {
    thread_builder: thread::Builder,
    stra_group: StrategyGroup,
}

impl StrategyManager {
    fn new(group: StrategyGroup) -> Self {
        Self {
            thread_builder: thread::Builder::new(),
            stra_group: group,
        }
    }
}


fn simulate_send(tx: Sender<TradeData>, n_thread: u32) {
    for i in 0..n_thread {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut n = 0;
            loop {
                let rand_value = rand::thread_rng().gen_range(0, 1000);
                n = n + 1;
                println!("rand_value:{:?} n:{:?} thread id :{:?}", rand_value, n, i);
                thread::sleep(Duration::from_millis(300));
                match rand_value {
                    0..=99 => tx.send(TradeData::RtnMarketData("IC".to_string())).unwrap(),
                    100..=299 => tx.send(TradeData::RtnMarketData("IF".to_string())).unwrap(),
                    300..=399 => tx.send(TradeData::RtnMarketData("cu".to_string())).unwrap(),
                    400..=599 => tx.send(TradeData::RtnMarketData("ag".to_string())).unwrap(),
                    600..=699 => tx.send(TradeData::RtnOrder("IC".to_string())).unwrap(),
                    700..=749 => tx.send(TradeData::RtnOrder("IF".to_string())).unwrap(),
                    750..=799 => tx.send(TradeData::RtnOrder("cu".to_string())).unwrap(),
                    800..=899 => tx.send(TradeData::RtnOrder("ag".to_string())).unwrap(),
                    900..=919 => tx.send(TradeData::RtnTrade("IC".to_string())).unwrap(),
                    920..=939 => tx.send(TradeData::RtnTrade("IF".to_string())).unwrap(),
                    940..=959 => tx.send(TradeData::RtnTrade("cu".to_string())).unwrap(),
                    _ => tx.send(TradeData::RtnTrade("ag".to_string())).unwrap(),
                };
            }
        });
    }
}

fn dispatch_data(rx: &Receiver<TradeData>, stra_group: &StrategyGroup) {
    //let (tx,rx) = channel();
    let strategys = &*stra_group.stra_list;
    loop {
        let ref value = rx.recv().unwrap();
        match value {
            TradeData::RtnMarketData(d) => {
                for strategy in strategys {
                    if strategy.stra_instructions.contains(&d) {
                        strategy.on_rtn_market_data(value);
                    }
                }
            }
            TradeData::RtnOrder(e) => {
                for strategy in strategys {
                    if strategy.stra_instructions.contains(&e) {
                        strategy.on_rtn_order(value);
                    }
                }
            }
            TradeData::RtnTrade(f) => {
                for strategy in strategys {
                    if strategy.stra_instructions.contains(&f) {
                        strategy.on_rtn_trade(value)
                    }
                }
            }
        }
    }
}

fn generate_strategy_manager() -> StrategyManager {
    let strategy_01 = Strategy::new("DSCJ".to_string(), vec!["IC".to_string(), "IF".to_string()]);
    let strategy_02 = Strategy::new("WSDJ".to_string(), vec!["IF".to_string()]);
    let strategy_03 = Strategy::new("TTTT".to_string(), vec!["ag".to_string(), "cu".to_string()]);
    let stra_group = StrategyGroup::new(vec![strategy_01, strategy_02, strategy_03]);
    StrategyManager::new(stra_group)
}

fn test() {
    //模拟生成相关的策略、策略group、策略管理者
    let stra_manager = generate_strategy_manager();
    // 模拟thost
    let (tx, rx) = channel::<TradeData>();
    //模拟多线程异步进行接收thost相关的行情等信息
    simulate_send(tx, 10);
    println!("main=>");

    dispatch_data(&rx, &stra_manager.stra_group);
    thread::sleep(Duration::from_millis(500000));
}

