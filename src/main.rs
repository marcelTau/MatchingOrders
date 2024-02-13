use std::{collections::BinaryHeap, io, str::FromStr};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Order {
    id: usize,
    order_type: OrderType,
    price: u32,
    quantity: u32,
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.price.cmp(&self.price)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Order {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let id: usize = parts[0].replace(':', "").parse()?;
        let order_type = match parts[1] {
            "Buy" => OrderType::Buy,
            "Sell" => OrderType::Sell,
            _ => return Err("OrderType not valid".into()),
        };

        let quantity: u32 = parts[2].parse()?;
        let price: u32 = parts[5].parse()?;

        Ok(Order {
            id,
            order_type,
            quantity,
            price,
        })
    }
}

struct Trade {
    buy_id: usize,
    sell_id: usize,
    price: u32, // this should be the sell price.
    quantity_traded: u32,
}

impl std::fmt::Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Trade: {} BTC @ {} USD between {} and {}",
            self.quantity_traded, self.price, self.buy_id, self.sell_id
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sell_orders = BinaryHeap::new();

    loop {
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).expect("Read line failed");

        if bytes_read == 0 {
            break;
        }

        let order = Order::from_str(&input)?;

        // If it's a sell order, store it and continue
        if order.order_type == OrderType::Sell {
            sell_orders.push(order);
            continue;
        } 

        let mut buyer = order;

        while let Some(seller) = sell_orders.peek() {
            if seller.price <= buyer.price {
                let quantity_traded = std::cmp::min(buyer.quantity, seller.quantity);

                let t = Trade {
                    buy_id: buyer.id,
                    sell_id: seller.id,
                    price: seller.price,
                    quantity_traded,
                };

                println!("{t}");

                if seller.quantity - quantity_traded == 0 {
                    sell_orders.pop();
                } else {
                    let mut seller = sell_orders.peek_mut().unwrap();
                    seller.quantity -= quantity_traded;
                }

                buyer.quantity -= quantity_traded;
                if buyer.quantity == 0 {
                    break;
                }
            } else {
                // There is no seller that sells at the price of the buyer.
                // Discard the buy order.
                break;
            }
        }
    }
    Ok(())
}
