use std::thread;
use std::sync::mpsc::channel;
// use std::collections::VecDeque;
use std::time::Duration;
use rand::Rng;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
// use std::thread::Thread;
use std::sync::{Arc, Mutex};
// use std::marker::Send;
// use std::cell::RefCell;

fn main() {
    test();
}
# [derive(Debug,Clone)]
struct RtnMarketData(String);
# [derive(Debug,Clone)]
struct RtnOrder(String);
# [derive(Debug,Clone)]
struct RtnTrade(String);


# [derive(Debug,Clone)]
enum TradeData{
    RtnMarketData(String),
    RtnOrder(String),
    RtnTrade(String),
}
trait GetId {
    fn get_request_id(&self)-> u32;
}

struct Strategy{
    stra_name:String,
    receiver:Receiver<TradeData>,
    lock:Arc<Mutex<u32>>,
}
struct StrategyHandler{
    stra_name: String,
    //订阅的相关合约
    stra_instructions: Vec<String>,
    sender:Sender<TradeData>,
}
impl StrategyHandler{
    fn new(name:String,instrs:Vec<String>,send:Sender<TradeData>)->Self{
        Self{
            stra_name: name,
            stra_instructions: instrs,
            sender:send,
        }
    }
}

impl Strategy{
    fn new(name:String,recv:Receiver<TradeData>,lock:Arc<Mutex<u32>>)->Self{
        Self{
            stra_name:name,
            receiver:recv,
            lock:lock,
        }
    }
    fn spawn(self){
        thread::spawn(move||{
            loop{
                let  value = self.receiver.recv().unwrap();
                match value {
                    TradeData::RtnMarketData(_) =>{
                        self.on_rtn_market_data(value);
                    },
                    TradeData::RtnOrder(_) =>{
                        self.on_rtn_order(value);
                    },
                    TradeData::RtnTrade(_) =>{
                        self.on_rtn_trade(value);
                    },
                }
            };
        });
    }
    fn on_rtn_market_data(&self, md:TradeData){
        println!(" -> strategy:{:?} RtnMarketData=> recv:{:?}",self.stra_name,md);
        let rand_value:u64 = rand::thread_rng().gen_range(200,500);
        // 获取唯一ID，策略逻辑在此
        if rand_value < 400 {
            println!("触发交易信号：{:?} id:{:?}",rand_value,self.get_request_id());
        }
        thread::sleep(Duration::from_millis(rand_value));
    }
    fn on_rtn_order(&self, order:TradeData){
        println!(" -> strategy:{:?} RtnOrder => recv:{:?}",self.stra_name,order);
        let rand_value:u64 = rand::thread_rng().gen_range(200,500);
        // 获取唯一ID
        if rand_value < 10 {
            println!("触发OnRtnOrder信号：{:?} id:{:?}",rand_value,self.get_request_id());
        }
    }
    fn on_rtn_trade(&self, trade:TradeData){
        println!(" -> strategy:{:?} RtnTrade=> recv:{:?}",self.stra_name,trade);
        let rand_value:u64 = rand::thread_rng().gen_range(200,500);
        // 获取唯一ID
        if rand_value < 300 {
            println!("触发OnRtnTrade信号：{:?} id:{:?}",rand_value,self.get_request_id());
        }
    }
}

impl GetId for Strategy{
    fn get_request_id(&self)-> u32{
        let lock = self.lock.clone();
        let temp = *lock.lock().unwrap();
        *lock.lock().unwrap()= temp +1 ;
        println!("request_ID:{:?}",*lock.lock().unwrap());
        temp+1
    }
}

struct StrategyGroup{
    stra_list : Vec<Strategy>,
}
impl StrategyGroup{
    fn new(list:Vec<Strategy>) -> Self{
        Self{
            stra_list:list,
        }
    }
}
struct StrategyManager{
    stra_group: StrategyGroup,
}

impl StrategyManager{
    fn new(group: StrategyGroup)->Self{
        Self{
            stra_group: group,
        }
    }
}

fn simulate_send(tx:Sender<TradeData>,n_thread:u32){
    for i in 0..n_thread {
        let tx = tx.clone();
        thread::spawn(move||{
            let mut n = 0;
            loop{
                let rand_value = rand::thread_rng().gen_range(0, 1000);
                n = n + 1;
                println!("thost send info: thread id :{:?} ,次数 {:?} ",i,n);
                thread::sleep(Duration::from_millis(500));
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
    //thost::new(tx).thost_thread_builder.spawn()
}
fn dispatch_data(rx:&Receiver<TradeData>,_handlers: Vec<StrategyHandler>){
    let handlers = &_handlers;
    loop{
        let  ref value = rx.recv().unwrap();
        match value {
            TradeData::RtnMarketData(d) =>{
                for  handler in handlers {
                    if handler.stra_instructions.contains(&d){
                        //strategy.on_rtn_market_data(value)
                        let tx = handler.sender.clone();
                        tx.send(TradeData::RtnMarketData(d.to_string())).unwrap();
                        println!("dispatch:{:?}",d);
                    }
                }
            },
            TradeData::RtnOrder(e) =>{
                for  handler in handlers {
                    if handler.stra_instructions.contains(&e){
                        let tx = handler.sender.clone();
                        tx.send(TradeData::RtnOrder(e.to_string())).unwrap();
                        println!("dispatch:{:?}",e);
                    }
                }
            },
            TradeData::RtnTrade(f) =>{
                for  handler in handlers {
                    if handler.stra_instructions.contains(&f){
                        let tx = handler.sender.clone();
                        tx.send(TradeData::RtnTrade(f.to_string())).unwrap();
                        println!("dispatch:{:?}",f);
                    }
                }
            },
        }
    }
}
fn strategy_init()-> Vec<StrategyHandler> {
    //创建管道
    let (tx_01,rx_01) = channel::<TradeData>();
    let (tx_02,rx_02) = channel::<TradeData>();
    let (tx_03,rx_03) = channel::<TradeData>();
    //id
    let id = Arc::new(Mutex::new(0_u32));
    //线程管道发送对象
    let stra_handler_01 = StrategyHandler::new("DSCJ".to_string(),vec!["IC".to_string(),"IH".to_string()],tx_01);
    //线程管道接收对象
    let strategy_01 = Strategy::new("DSCJ".to_string(),rx_01,id.clone());
    println!("a");
    strategy_01.spawn();
    println!("b");
    let stra_handler_02 = StrategyHandler::new("TTTT".to_string(),vec!["IC".to_string(),"IF".to_string()],tx_02);
    let strategy_02 = Strategy::new("TTTT".to_string(),rx_02,id.clone());
    strategy_02.spawn();
    let stra_handler_03 = StrategyHandler::new("WSDJ".to_string(),vec!["ag".to_string(),"cu".to_string()],tx_03);
    let strategy_03 = Strategy::new("WSDJ".to_string(),rx_03,id.clone());
    strategy_03.spawn();
    vec![stra_handler_01,stra_handler_02,stra_handler_03]
}
// 等待=>


fn test(){
    //模拟生成相关的策略、策略group、策略管理者
    let handlers :Vec<StrategyHandler> = strategy_init();
    //模拟thost
    let (tx,rx) = channel::<TradeData>();
    //模拟N个多线程异步进行接收thost相关的行情等信息
    simulate_send(tx,2);
    println!("main=>");
    dispatch_data(&rx,handlers);
    thread::sleep(Duration::from_millis(500000));
}

fn main(){
}