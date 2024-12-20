ast = {
    "Expr": [
        "Unary: Token operator, Box<Expr> right",
        "Binary: Box<Expr> left, Token operator, Box<Expr> right",
        "Literal: Token value",
        "Grouping: Box<Expr> expression",
        "Variable: Token name",
        "Assign: Token name, Box<Expr> value",
        "Logical: Box<Expr> left, Token operator, Box<Expr> right",
    ],
    "Stmt": [
        "Block: Vec<Stmt> statements",
        "Expression: Box<Expr> expression",
        "Print: Box<Expr> expression",
        "Var: Token name, Option<Box<Expr>> initializer",
        "IfStmt: Box<Expr> condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch",
    ]
}

lines = []
def add_line(line):
    lines.append(line + "\n")

# Imports
add_line("use crate::token::Token;")
add_line("")

for base in ast.keys():
    nodes = ast[base]

    # Create enum
    add_line(f"pub enum {base} {{")
    for node in nodes:
        name = node.split(":")[0]
        add_line(f"\t{name}({name}),")
    add_line("}")

    add_line("")

    # Create structs
    for node in nodes:
        name = node.split(":")[0]
        fields = node.split(":")[1].split(",")

        add_line(f"pub struct {name} {{")
        for field in fields:
            field_type, field_name = field.split()
            add_line(f"\tpub {field_name}: {field_type},")
        add_line("}")

        add_line("")

        # Create impl for struct
        add_line(f"impl {name} {{")
        params = ""
        for field in fields:
            field_type, field_name = field.split()
            params += f"{field_name}: {field_type}, "
        params = params.rstrip(", ")
        add_line(f"\tpub fn new({params}) -> Self {{")
        add_line("\t\tSelf {")
        for field in fields:
            field_name = field.split()[1]
            add_line(f"\t\t\t{field_name}: {field_name},")
        add_line("\t\t}")
        add_line("\t}")
        add_line("}")
        
        add_line("")

    # Create Visitor
    add_line(f"pub trait {base}Visitor {{")
    add_line("\ttype Result;")
    add_line("")
    for node in nodes:
        name = node.split(":")[0]
        add_line(f"\tfn visit_{name.lower()}(&mut self, {name.lower()}: &{name}) -> Self::Result;")
    add_line("}")

    add_line("")

    # Create Accept for Visitor
    add_line(f"pub trait {base}Accept {{")
    add_line(f"\tfn accept<V: {base}Visitor>(&self, visitor: &mut V) -> V::Result;")
    add_line("}")

    add_line("")

    # Impl Accept
    add_line(f"impl {base}Accept for {base} {{")
    add_line(f"\tfn accept<V: {base}Visitor>(&self, visitor: &mut V) -> V::Result {{")
    add_line("\t\tmatch self {")
    nodes = ast[base]
    for node in nodes:
        name = node.split(":")[0]
        add_line(f"\t\t\tSelf::{name}(x) => visitor.visit_{name.lower()}(x),")
    add_line("\t\t}")
    add_line("\t}")
    add_line("}")

# Convert tabs to space
lines = [line.replace("\t", "    ") for line in lines]

with open("src/ast.rs", "w") as file:
    file.writelines(lines)