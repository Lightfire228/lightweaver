#!/bin/env python

from pathlib import Path
import re

me       = Path(__file__).resolve()
proj_dir = me.parent.parent

ast_dir  = proj_dir / 'src/script/ast'

def main():
    
    expr = [
        Ast('Assign',        ['name:       Token',        'value:       Box<Expr>']),
        Ast('Grouping',      ['expression: Box<Expr>']),
        Ast('Instantiation', ['type_:      Token',        'body:        Box<Body>']),
        Ast('Body',          ['properties: Vec<Property>']),
        Ast('Property',      ['name:       Token',        'initializer: Box<Expr>']),
        Ast('Literal',       []),
        Ast('Logical',       ['left:       Box<Expr>',    'operator:    Token', 'right: Box<Expr>']),
        Ast('Variable',      ['name:       Token']),
    ]

    stmt = [
        Ast('Block',      ['statements: Vec<Stmt>']),
        Ast('Expression', ['expression: Box<Expr>']),
        Ast('Let',        ['name:       Token',    'initializer: Box<Expr>']),
    ]

    define_ast('expr', expr)
    define_ast('stmt', stmt, imports=[
        'use super::expr::Expr;'
    ])

def define_ast(base_name: str, types: list['Ast'], imports: list[str] = []):

    base_name   = base_name.lower()
    struct_name = base_name.capitalize()
    
    file = ast_dir / f'{base_name}.rs'

    width = max([len(x.class_name) for x in types])

    def format(name: str) -> str:
        return name + ( ' ' * (width - len(name)) ) + f'({name})'
    
    imports.append(
        'use crate::script::tokens::Token;'
    )

    
    text = [
        '// Auto generated code. Edit scripts/generate_ast.py instead',
        '',
        *imports,
        '',
        f'pub enum {struct_name} {{',
        *[
            f'    {format(type_.class_name)},'
            for type_ in types
        ],
        '}',
        '',
        *[
            define_type(type_) 
            for type_ in types
        ],
        '',
        define_visit(types),

    ]

    file.touch()
    file.write_text('\n'.join(text))

def define_type(type_: 'Ast'):

    # remove the extra spaces
    def normalize(p: str) -> tuple[str, str]:
        return re.sub(r' +', ' ', p).split(':')

    props = { k:normalize(k) for k in type_.args }
    names = [ x[0] for x in props.values() ] or ['']
    width = max([ len(x) for x in names ])

    # align the prop types vertically 
    def format(arg_name) -> str:
        prop = props[arg_name]

        name = prop[0]
        name = f'{name}:' + ' ' * (width - len(name))

        return name + prop[1]

    return '\n'.join([
        f'pub struct {type_.class_name} {{',
        *[
            f'    {format(prop)},'
            for prop in type_.args
        ],
        '}',
        '',
    ])

def define_visit(types: list['Ast']):

    width = max([len(x.class_name) for x in types])

    def func(type_: 'Ast') -> str:
        spaces = ' ' * (width - len(type_.class_name))

        func_name = type_.class_name.lower()
        return f'fn visit_{func_name}{spaces}(&mut self, x: &{type_.class_name}){spaces} -> T;'


    return '\n'.join([
        'pub trait Visitor<T> {',
        *[
            f'    {func(type_)}'
            for type_ in types
        ],
        '}',
    ])


class Ast():

    def __init__(self, class_name: str, args: list[str]):
        self.class_name = class_name
        self.args       = args

if __name__ == '__main__':
    main()