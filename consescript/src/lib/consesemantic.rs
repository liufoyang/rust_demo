use super::consesyntax::AST_Node;
use consesyntax::AST_Node_Type;

#[derive(Clone)]
enum Symbol_Type {
    int,
    dubbo,
    float,
    string,
    funtion,
    bool,
    undefine,
}

impl Symbol_Type {
    pub fn fromStr(name:&str) -> Symbol_Type{
        if (name == "int") {
            return Symbol_Type::int;
        }

        if (name == "dubbo") {
            return Symbol_Type::dubbo;
        }

        if (name == "string") {
            return Symbol_Type::string;
        }

        if (name == "funtion") {
            return Symbol_Type::funtion;
        }

        if (name == "float") {
            return Symbol_Type::float;
        }

        return Symbol_Type::undefine;
    }
}

struct Symbol {
    name:String,
    stype:Symbol_Type,
    scope_index: usize,
}

impl Symbol {
    pub fn new(_name:&str, _stype:Symbol_Type, _scope_index:usize) -> symbol {
        let symbol = Symbol{
            name:_name.to_string(),
            stype:_stype,
            scope_index: _scope_index,
        };

        return symbol;
    }

    pub fn get_stype(&self) -> Symbol_Type {
        return self.stype.clone();
    }
}



struct Block_Scope {
    symbol_list:Vec<Symbol>,
    parent_index:Option<usize>,
}

impl Block_Scope {
    pub fn new(_parent_index:Option<usize>) -> Block_Scope {
        let scope = Block_Scope {
            symbol_list:Vec::new(),
            parent_index:_parent_index,
        };
        return scope;
    }

    pub fn get_parent_index(&self) -> Option<usize> {
        return self.parent_index.clone();
    }

    fn containCurrentScopeSymbol(&self, name:&str) -> bool {
        for symbol in &self.symbol_list {
            if symbol.name.as_str() == name  {
                return true;
            }
        }

        return false;
    }

    fn findSymbol(&self, name:&str) -> Option<Symbol_Type> {
        for symbol in &self.symbol_list {
            if symbol.name.as_str() == name  {
                return Some(symbol.stype.clone());
            }
        }

        return None;

    }

    fn addSymbol(&mut self, name:&str, stype:Symbol_Type, scope_index:usize) {
        let has_symbol = self.containCurrentScopeSymbol(name);

        if has_symbol {
            panic!("repeate symbol defined for {}", name);
        }

        let symbol = Symbol {
            name:name.to_string(),
            stype:stype,
            scope_index: scope_index,
        };

        self.symbol_list.push(symbol);

    }
}


fn findSymbolFromAvaScope (scope_list:&Vec<Scope>, _name:&str, scope_index:usize) ->Option<Symbol_Type> {
    let currunt_scope = scope_list.get(scope_index).unwrap();

    let mut symbol_type_result = currunt_scope.findSymbol(_name.clone());

    if symbol_type_result.is_some() {
        return symbol_type_result;
    }

    let mut parent_index_result = currunt_scope.get_parent_index();
    while parent_index_result.is_some() {
        let scope = scope_list.get(scope_index).unwrap();
        symbol_type_result = scope.findSymbol(_name.clone());

        if symbol_type_result.is_some() {
            break;
        }
    }

    return symbol_type_result;

}

struct Semantic_Context {
    symbol_list:Vec<Symbol>,
    scope_list:Vec<Scope>,
    node_scope_map:Map<i32, usize>,
    node_symbol_map:Map<i32, usize>,
    node_primary_type_map:Map<i32, Symbol_Type>,
}

pub fn semanticParse(root:AST_Node) -> Semantic_Context {

}

trait AST_Tree_Processor {
    fn process_enter_node(&mut self, node: & AST_Node, semantic_context:&mut Semantic_Context);

    fn process_exit_node(&mut self, node: & AST_Node, semantic_context:&mut Semantic_Context);
}

pub fn walk_AST_tree(root: & AST_Node, tree_process: &mut AST_Tree_Processor) {

    let mut semantric_context = Semantic_Context{
        symbol_list:Vec::new(),
        scope_list:Vec::new(),
        node_scope_map:HashMap::new(),
        node_symbol_map:HashMap::new(),
        node_primary_type_map:HashMap::new(),
    };

    walk_sigle_AST_node(root, tree_process, semantic_context);
}

pub fn walk_sigle_AST_node(node: & AST_Node, tree_process: &mut AST_Tree_Processor, semantic_context:&mut Semantic_Context) {
    tree_process.process_enter_node(node, &mut semantric_context);

    for childNode in node.getChildren() {
        walk_sigle_AST_node(childNode, tree_process, semantic_context);
    }

    tree_process.process_exit_node(node, &mut semantric_context);
}

///作用域的解析
///
struct Scope_Resolver{
    scope_list:Vec<Block_Scope>,
    current_index:usize,
}

impl Scope_Resolver {
    pub fn new() -> Scope_Resolver{
        let resolver = Scope_Resolver{
            scope_list:Vec::new(),
            current_index:usize::max_id(),
        };
    }

    pub fn pushScope(&mut self, scope:Block_Scope) {
        self.scope_list.push(scope);

        if self.current_index == usize::max_id() {
            self.current_index = 0;
        } else {
            self.current_index +=1;
        }
    }

    pub fn popScope(&mut self) -> Option<usize> {
        if( self.scope_list.len() > 0) {
            self.current_index -=1;
            return Some(self.current_index +1);
        } else {
            return None;
        }
    }
}

impl AST_Tree_Processor for Scope_Resolver {
    fn process_enter_node(&mut self, node: &AST_Node, semantic_context: &mut Semantic_Context) {

        // not new scope
        if node.get_type() != AST_Node_Type::Function
            && node.get_type() != AST_Node_Type::ConditionBlockStmt
            && node.get_type() != AST_Node_Type::StatementBlock{

            return;
        }
        let mut scope = Block_Scope::new(current_index.clone());

        semantic_context.node_scope_map.insert(node.get_id(), self.scope_list.len());
        self.pushScope(scope);

    }

    fn process_exit_node(&mut self, node: &AST_Node, semantic_context: &mut Semantic_Context) {

        if (node.get_id() == 0) {
            semantic_context.scope_list = self.scope_list;
            return;
        }
        // not new scope
        if node.get_type() != AST_Node_Type::Function
            && node.get_type() != AST_Node_Type::ConditionBlockStmt
            && node.get_type() != AST_Node_Type::StatementBlock{

            return;
        }
        self.popScope(scope);
    }
}

///作用域的解析
///
struct Variable_Resolver{
    symbol_list:Vec<Symbol>,
}

impl Variable_Resolver {
    pub fn new() -> Variable_Resolver{
        let variable_resolver = Variable_Resolver{
            symbol_list:Vec::new(),
        };

        return variable_resolver;
    }

    pub fn addSymbol(&mut self, symbol:Symbol) -> usize {
        self.symbol_list.push(symbol);

        return self.symbol_list.len() -1;
    }
}

impl AST_Tree_Processor for Variable_Resolver {
    fn process_enter_node(&mut self, node: &AST_Node, semantic_context: &mut Semantic_Context) {
        // not new scope
        if node.get_type() != AST_Node_Type::Declaration{
            return;
        }

        let identifer_node = node.get_children().get(0).unwrap();
        let symbol_name = identifer_node.get_value().to_string();

        let mut symbol_type = Symbol_Type::undefine;
        if node.get_value().len()!= 0 {
            symbol_type = Symbol_Type::fromStr(node.get_value());
        }

        let scope_index = semantic_context.node_scope_map.get(node.get_id()).unwrap().clone();

        let symbol = Symbol::new(symbol_name, symbol_type.clone(), scope_index);

        let symbol_index = self.addSymbol(symbol);

        semantic_context.node_symbol_map.insert(node.get_id(), symbol_index);
        semantic_context.node_primary_type_map.insert(node.get_id(), symbol_type);
    }

    fn process_exit_node(&mut self, node: &AST_Node, semantic_context: &mut Semantic_Context) {
        unimplemented!()
    }
}
///作用域的解析
///
struct PrimaryType_Resolver{

}

impl AST_Tree_Processor for PrimaryType_Resolver {
    fn process_enter_node(&mut self, node: &AST_Node, semantic_context: &mut Semantic_Context) {
        if node.get_type() == AST_Node_Type::Declaration{
            return;
        }

        let mut symbol_type = Symbol_Type::undefine;
        if node.get_type() == AST_Node_Type::TypeType{
            symbol_type = Symbol_Type::fromStr(node.get_value());
        }

        // 基本类型节点， 按推断的类型，下节点是type
        if node.get_type() == AST_Node_Type::Primary || node.get_type() == AST_Node_Type::ParameterDefine {
            let type_node = node.get_children().get(0).unwrap();

            symbol_type = Symbol_Type::fromStr(type_node.get_value());
        }

        // identifier 就取变量的定义类型
        if node.get_type() == AST_Node_Type::Identifier {
            let scope_index:usize = semantic_context.node_scope_map.get(node.get_id()).unwrap();
            let scope = semantic_context.scope_list.get(symbol_index).unwrap();

            let symbol_result = scope.findSymbol(node.get_value());

            if symbol_result.is_some() {
                symbol_type = symbol_result.unwrap()..get_stype();
            }

        }

        if node.get_type() == AST_Node_Type::ConditionExpression {
            symbol_type = Symbol_Type::bool;
        }

        if node.get_type() == AST_Node_Type::StringLiteral {
            symbol_type = Symbol_Type::string;
        }

        if node.get_type() == AST_Node_Type::DoubleLiteral {
            symbol_type = Symbol_Type::dubbo;
        }

        if node.get_type() == AST_Node_Type::IntLiteral {
            symbol_type = Symbol_Type::int;
        }

        semantic_context.node_primary_type_map.insert(node.get_id(), symbol_type);

    }

    fn process_exit_node(&mut self, node: &_, semantic_context: &mut Semantic_Context) {
        let mut symbol_type = Symbol_Type::undefine;
        if node.get_type() == AST_Node_Type::Multiplicative || node.get_type() == AST_Node_Type::Additive {
            let left_node = node.get_children().get(0).unwrap();
            let left_type = semantic_context.node_primary_type_map.get(left_node.get_id()).unwrap();
            if left_type!= Symbol_Type::int && left_type!= Symbol_Type::dubbo{
                painc!("error the mul value {}", left_type);
            }

            let right_node = node.get_children().get(1).unwrap();
            let right_type = semantic_context.node_primary_type_map.get(right_node.get_id()).unwrap();

            if right_type!= Symbol_Type::int && right_type!= Symbol_Type::dubbo{
                painc!("error the mul value {}", right_type);
            }

            if right_type == Symbol_Type::dubbo||left_type == Symbol_Type::dubbo {
                symbol_type = Symbol_Type::dubbo;
            } else {
                symbol_type = Symbol_Type::int;
            }

            // 程序块的属性，就取return或者最后一句语句的类型
            if node.get_type() == AST_Node_Type::StatementBlock {
                let statement_nodes = node.get_children();
                // TODO 检查所有return类型。
            }
        }


    }
}