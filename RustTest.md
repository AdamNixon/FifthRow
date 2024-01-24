# Fifth Row Rust test

This test requires use of the Rust programming language. You may not have used Rust before - this is fine! There test is structured so that it doesn't require a huge amount of Rust expertise to do, but you will be required to learn some things about Rust.

You can write and test the code using some online resources like rust-playground.com, or you can use an editor like VSCode with the Rust Analyser plugin.

Fortunately Rust has really good documentation online - there are a wealth of resources available. Here are the places you are most likely to find useful for this test.

https://www.rust-lang.org/learn - this is the getting started page for learning rust - the ebooks "The Rust programming language" and "Rust by Example" are good points of reference to start you off
https://doc.rust-lang.org/std/index.html - this is the documentation for the rust standard library

The useful parts of Rust for a quick start are: structs and enums, mutability, move semantics, pattern matching and derive attributes. You may also want to look at how to write tests, and how to use cargo to build code, run tests and create new projects (crates).

## Project: Matching Orders

Suppose you run a trading site for bitcoin to match orders to buy against orders to sell. We want to emit trade messages every time a buy order trades with a sell order.

When an order to sell comes in we need to store them in an "order book" until an order to buy comes in.

As soon as an order to buy comes in, it should match against one (or more) of the existing orders to sell. The matching algorithm is as follows:

1. Find the sell order with the lowest price. 
2. Compare it to the price of the buy order. 
3. If that sell price is less than or equal to the buy price, we emit a trade message and reduce the quantities on both orders. 
4. If the sell order reaches zero quantity, it is removed. 
5. Repeat from step 1 until there are no more orders matching or the buy order has been completely fulfilled.

Note that in the above, if there aren't enough sell orders in the order book, then we discard whatever is left on the buy order.


Each order consists of the following information:

order type: buy or sell.
price: [1, 99999],
quantity: [1, 999],


To get you started, here are some of the structures you'll need. Feel free to add attributes, make changes as required.

```rust
enum OrderType {
    Buy,
    Sell,
}

struct Order {
    id: usize,
    order_type: OrderType,
    price: u32,
    quantity: u32
}

struct Trade {
    buy_id: usize,
    sell_id: usize,
    price: u32, // this should be the sell price.
    quantity_traded: u32
}
```

## Inputs and Outputs

You should take orders from stdin. Each line of input will be in the form "{id}: {Buy|Sell} {quantity} BTC @ {price}".

You should output to stdout an array of trades of the form "Trade: {quantity} BTC @ {price} between {buy_id} and {sell_id}"

## Examples

Here are some example orders that illustrate how it should work. Note that these are not exhaustive of every possible case.

1. Simple trade:

Input:

```
1: Sell 100 BTC @ 5000 USD
2: Buy 50 BTC @ 6000 USD
```

Output:

```
Trade: 50 BTC @ 5000 USD between 2 and 1
```

Notice that the price is the lower of the two prices (the sell price),

2. Multiple Trades for a single order

Input:

```
1: Sell 100 BTC @ 5001 USD
2: Sell 25 BTC @ 5000 USD
3: Buy 50 BTC @ 6000 USD
```

Output:

```
Trade 25 BTC @ 5000 USD between 3 and 2
Trade 25 BTC @ 5001 between 3 and 1
```

We have to match 2 orders to fill the entire order.
Notice it matches the order with lowest sell price first.

3. Not enough orders

Input:

```
1: Sell 75 BTC @ 5000 USD
2: Buy 50 BTC @ 6000 USD
3: Buy 50 BTC @ 6000 USD
```

Output:

```
Trade 50 BTC @ 5000 USD between 2 and 1
Trade 25 BTC @ 5000 USD between 3 and 1
```

Note that because we used up some of the sell order with the first buy order,
we don't have enough to fill up the second.

4. Order of orders is important!

Input:

```
1: Sell 75 BTC @ 5000 USD
2: Buy 100 BTC @ 6000 USD
3: Sell 75 BTC @ 5000 USD
4: Buy 50 BTC @ 6000 USD
```

Output:

```
Trade 75 BTC @ 5000 USD between 2 and 1
Trade 50 BTC @ 5000 USD between 4 and 3
```

Note that the first order does not get completely filled, because there wasn't enough quantity on the sell orders at the time the buy order was received. The second sell order (order 3) arrives later, so cannot be taken into account.
