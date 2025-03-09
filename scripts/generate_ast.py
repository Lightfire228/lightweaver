#!/bin/env python

from pathlib import Path

me       = Path(__file__).resolve()
proj_dir = me.parent

ast_dir  = me / 'src/script/ast'

def main():
    
    expr = [
        Ast('Assign',        ['name:       Token',        'value:       Expr']),
        Ast('Grouping',      ['expression: Expr']),
        Ast('Instantiation', ['type_:      Token',        'body:        Body']),
        Ast('Body',          ['properties: Vec<Property>']),
        Ast('Property',      ['name:       Token',        'initializer: Expr']),
        Ast('Literal',       []),
        Ast('Logical',       ['left:       Expr',         'operator:    Token', 'right: Expr']),
        Ast('Variable',      ['name:       Token']),
    ]

    stmt = [
        Ast('Block',      ['statements: Vec<Stmt>']),
        Ast('Expression', ['expression: Expr']),
        Ast('Let',        ['name:       Token',    'initializer: Expr']),
    ]

def defineAst(base_name: str, ast: 'Ast'):
    
    file = ast_dir / f'{base_name}.rs'
    



class Ast():

    def __init__(self, class_name: str, args: list[str]):
        self.class_name = class_name
        self.args       = args

if __name__ == '__main__':
    main()