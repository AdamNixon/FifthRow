use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::io::{self, BufRead};

#[derive(Hash, Eq, PartialEq, Clone)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Order {
    id: usize,
    order_type: OrderType,
    price: u32,
    quantity: u32,
}

struct Trade {
    buy_id: usize,
    sell_id: usize,
    price: u32, // this should be the sell price.
    quantity_traded: u32,
}

fn line2order(l: &str) -> Result<Order, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = l.split_whitespace().collect();
    if parts.len() != 7 || parts[6] != "USD" || parts[4] != "@" || parts[3] != "BTC" {
        return Err("Invalid input format".into());
    }
    // first part is id
    let id: usize = parts[0].splitn(2, ':').next().unwrap().parse()?;
    // second is Buy/Sell
    let order_type: OrderType = match parts[1] {
        "Buy" => OrderType::Buy,
        "Sell" => OrderType::Sell,
        _ => return Err("Invalid order type".into()),
    };
    // quantity
    let quantity: u32 = parts[2].parse()?;
    let price: u32 = parts[5].parse()?;

    Ok(Order {
        id: id,
        order_type: order_type,
        price: price,
        quantity: quantity,
    })
}

fn processorder(
    order: Order,
    book: &mut PriorityQueue<Order, Reverse<u32>>,
    trades: &mut Vec<Trade>,
) {
    if order.order_type == OrderType::Buy {
        let mut req_volume: u32 = order.quantity;
        while req_volume > 0 && !book.is_empty() {
            let mut top = book.pop().unwrap();
            if top.0.price > order.price {
                // bid price is too low, order discarded
                book.push(top.0, top.1);
                break;
            }
            if top.0.quantity <= req_volume {
                // fully fill order
                let trade = Trade {
                    buy_id: order.id,
                    sell_id: top.0.id,
                    price: top.0.price,
                    quantity_traded: top.0.quantity,
                };
                // decrease order book's quantity by amount bought
                trades.push(trade);
                req_volume -= top.0.quantity;
            } else {
                // partial fill order
                let trade = Trade {
                    buy_id: order.id,
                    sell_id: top.0.id,
                    price: top.0.price,
                    quantity_traded: req_volume,
                };
                trades.push(trade);
                top.0.quantity -= req_volume;
                req_volume = 0;
                book.push(top.0, top.1);
            }
        }
    } else if order.order_type == OrderType::Sell {
        book.push(order.clone(), Reverse(order.price));
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    let mut processed: Vec<Trade> = Vec::new();
    // order book
    let mut obook = PriorityQueue::new();

    // read lines
    for line in lines {
        // for each time parse input
        let order: Order = line2order(&line).expect("Failed to parse order");
        processorder(order, &mut obook, &mut processed);
    }
    // Output completed orders
    for p in processed {
        println!(
            "Trade {} BTC @ {} USD between {} and {}",
            p.quantity_traded, p.price, p.buy_id, p.sell_id
        )
    }
    Ok(())
}
