extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

use std::fs;
use pest::iterators::Pair;

use std::rc::Rc;
use std::cell::RefCell;

enum SymbolType {
    VARIABLE,
    CONSTANT,
}

enum ValueType{
    INT,
    REAL,
    STRING,
    OTHER,
}

struct Value(i32, f32, str);

struct Symbol{
    symbol_name: String,
    symbol_type: SymbolType,
    value_type: ValueType,
    symbol_value: Box<Value>,
    symbol_scope: Vec<Symbol>,
}

struct BlockStruct {
    pub test_field: i32,
    symbols: Vec<Symbol>,
    child_blocks: Vec<Rc<RefCell<Box<Block>>>>,
    current_block: Option<Rc<RefCell<Box<Block>>>>,
    parent_block: Option<Rc<RefCell<Box<Block>>>>,
}

impl BlockStruct {
    fn new() -> Self {
        BlockStruct {
            test_field: 1,
            symbols: Vec::new(), 
            child_blocks: Vec::new(), 
            current_block: None,
            parent_block: None,
        }
    }
    // The structure don't know its location before created, so its environment has to init later.
    fn init_env(&mut self, current_block: Rc<RefCell<Box<Block>>>, parent_block: Option<Rc<RefCell<Box<Block>>>>) {
        self.current_block = Option::Some(current_block);
        self.parent_block = parent_block;
    }
    fn init_block_ptr_from_block(block: Box<Block>, parent_block: Option<Rc<RefCell<Box<Block>>>>) -> Rc<RefCell<Box<Block>>> {
        let block_ptr : Rc<RefCell<Box<Block>>> = Rc::new(RefCell::new(block));
        let block_struct = block_ptr.borrow().get_block_struct();
        block_struct.borrow_mut().init_env(block_ptr.clone(), parent_block);
        block_ptr
    }
    fn compile_child_blocks(&mut self, pair: Pair<Rule>) {
        for matched_pair in pair.into_inner() {
            match matched_pair.as_rule() {
                Rule::assign => {
                    println!("Matched?");
                    let parent_block: Option<Rc<RefCell<Box<Block>>>> = self.current_block.clone();
                    let assign_block = AssignBlock::new();
                    let assign_block_ptr = BlockStruct::init_block_ptr_from_block(Box::new(assign_block), parent_block);
                    assign_block_ptr.borrow_mut().compile(matched_pair);
                    self.mount_child_block(assign_block_ptr);
                },
                _ => {
                    println!("Other rules, not supported yet.");
                }
            }
        }
    }
    fn mount_child_block(&mut self, child_block: Rc<RefCell<Box<Block>>>) {
        self.child_blocks.push(child_block);
    }
}

// Block Trait
trait Block {
    fn new() -> Self where Self: Sized;
    fn compile(&mut self, pair: Pair<Rule>);
    fn run(&self);
    fn get_block_struct(&self) -> Rc<RefCell<BlockStruct>>;
}

// Assign Blocks
struct AssignBlock {
    block_struct: Rc<RefCell<BlockStruct>>, 
}

impl Block for AssignBlock{
    fn new() -> Self {
        println!("Constructing an Assign Block!");
        AssignBlock {
            block_struct: Rc::new(RefCell::new(BlockStruct::new())),
        }
    }
    fn compile(&mut self, pair: Pair<Rule>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::key => {
                    println!("{}", inner_pair);
                },
                Rule::expression => {
                    println!("{}", inner_pair);
                },
                _ => println!("Wrong in Assign Block"),
            }
        }
    }
    fn run(&self) {
        println!("Interesting...");
    }
    fn get_block_struct(&self) -> Rc<RefCell<BlockStruct>> {
        self.block_struct.clone()
    }
}

// Root Blocks
struct RootBlock {
    block_struct: Rc<RefCell<BlockStruct>>,
}

impl Block for RootBlock {
    fn new() -> Self{
        println!("Constructing a Declare Block!");
        RootBlock {
            block_struct: Rc::new(RefCell::new(BlockStruct::new())),
        }
    }
    fn compile(&mut self, pair: Pair<Rule>) {
        self.block_struct.borrow_mut().compile_child_blocks(pair);
    }
    fn run(&self) {
        println!("Interesting...");
    }
    fn get_block_struct(&self) -> Rc<RefCell<BlockStruct>> {
        self.block_struct.clone()
    }
}

fn main() {
    let unparsed_file_content = fs::read_to_string("test.pse").expect("cannot read file");
    let root_pair = MyParser::parse(Rule::blocks, &unparsed_file_content).unwrap().next().unwrap();

    println!("{}", root_pair);

    let root_block = RootBlock::new();
    let root_block_ptr = BlockStruct::init_block_ptr_from_block(Box::new(root_block), None);
    root_block_ptr.borrow_mut().compile(root_pair);
    root_block_ptr.borrow_mut().run();

    //parse_pair(root_pair);
}


