#![cfg_attr(feature = "nightly", feature(proc_macro_span))]
extern crate proc_macro;

#[path = "../../microserde/derive/src/macro_lib.rs"]
mod macro_lib; 
use macro_lib::{error, TokenBuilder, TokenParser};
use proc_macro::{TokenStream, Delimiter, Span};

#[derive(Clone, Copy, Debug)]
struct Lc {
    line: usize,
    column: usize
}

impl Lc{
    fn next_char(self)->Self{
        Self{line:self.line, column:self.column+1}
    }
}

fn delim_to_pair(delim:Delimiter)->(char, char){
    match delim{
        Delimiter::Brace=>('{','}'),
        Delimiter::Parenthesis=>('(',')'),
        Delimiter::Bracket=>('[',']'),
        Delimiter::None=>(' ',' '),
    }
}

#[cfg(feature = "nightly")]
fn tokenparser_to_string(parser: &mut TokenParser, span:Span, out: &mut String, last_end:&mut Option<Lc>) {
    
    fn lc_from_start(span:Span)->Lc{
        Lc{
            line:span.start().line,
            column:span.start().column
        }
    }
    
    fn lc_from_end(span:Span)->Lc{
        Lc{
            line:span.end().line,
            column:span.end().column
        }
    }
    
    fn delta_whitespace(now:Lc, needed:Lc, out: &mut String){
        
        if now.line == needed.line{
            for _ in now.column..needed.column{
                out.push(' ');
            }
        }
        else{
            for _ in now.line..needed.line{
                out.push('\n');
            }
            for _ in 0..needed.column{
                out.push(' ');
            }
        }
    }
    
    if last_end.is_none(){
        *last_end = Some(lc_from_start(span));
    }

    while !parser.eat_eot(){
        let span = parser.span().unwrap();
        if let Some(delim) = parser.open_group(){
            let (gs,ge) = delim_to_pair(delim);
            let start = lc_from_start(span);
            let end = lc_from_end(span);
            delta_whitespace(last_end.unwrap(), start, out);
            out.push(gs);
            *last_end = Some(start.next_char());
            tokenparser_to_string(parser, span, out, last_end);
            delta_whitespace(last_end.unwrap(), end, out);
            *last_end = Some(end);
            out.push(ge);
        }
        else{
            if let Some(tt) = &parser.current{
                let start = lc_from_start(span);
                delta_whitespace(last_end.unwrap(), start, out);
                out.push_str(&tt.to_string());
                *last_end = Some(lc_from_end(span));
            }
            parser.advance();
        }
    }
}

#[cfg(not(feature = "nightly"))]
fn tokenparser_to_string(parser: &mut TokenParser, span:Span, out: &mut String, _last_end:&mut Option<Lc>) {
    while !parser.eat_eot(){
        let span = parser.span().unwrap();
        if let Some(delim) = parser.open_group(){
            let (s,e) = delim_to_pair(delim);
            out.push(s);
            tokenparser_to_string(parser, span, out, last_end);
            out.push(e);
        }
        else{
            if let Some(tt) = &parser.current{
                out.push_str(&tt.to_string());
            }
            parser.advance();
        }
    }
}

#[proc_macro]
pub fn live_body(input: TokenStream) -> TokenStream {

    let mut parser = TokenParser::new(input);
    let mut tb = TokenBuilder::new();
    if let Some(cx) = parser.eat_any_ident(){
        if parser.eat_punct(','){
            let span = parser.span().unwrap();
            if parser.open_brace(){
                let mut s = String::new();
                tokenparser_to_string(&mut parser, span, &mut s, &mut None);
                tb.ident(&cx);
                tb.add(". add_live_body ( LiveBody {");
                tb.add("module_path :").ident_with_span("module_path",span).add("! ( ) . to_string ( ) ,");
                tb.add("file :").ident_with_span("file",span).add("! ( ) . to_string ( ) . replace ( ").string("\\").add(",").string("/").add(") ,");
                tb.add("line :").ident_with_span("line",span).add("! ( ) as usize ,");
                tb.add("column :").ident_with_span("column",span).add("! ( ) as usize ,");
                tb.add("code :").string(&s).add(" . to_string ( ) } )");
                return tb.end()
            }
        }
    }
    
    return error("Macro use syntax error, usage live_body!(cx, {...})")
 }
