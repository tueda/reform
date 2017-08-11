use structure::{Element,Func,IdentityStatementMode,Statement,Module,IdentityStatement,NumOrder,Program};
use nom::{digit,alpha,alphanumeric,GetInput};
use std;
use std::str;
use std::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

named!(builtin <String>, do_parse!(f: map_res!(alt_complete!(tag!("dd_") | tag!("delta_") 
	| tag!("theta_") | tag!("pow_") | tag!("nargs_")), std::str::from_utf8) >> (f.to_owned())));

// varname: alpha + alphanumeric
named!(varname <String>, do_parse!(
	first: map_res!(alpha, std::str::from_utf8) >>
  rest: opt!(complete!(map_res!(alphanumeric, std::str::from_utf8))) >>
	(
    match rest {
		Some(vv) => first.to_owned() + vv,
		None => first.to_owned()
	} )
	
));

named!(pub exprparen <Element>, ws!(delimited!(char!('('), expression, char!(')'))));

named!(minusexpr <Element>, do_parse!(ws!(tag!("-")) >>
	g: map!(alt_complete!(term | element),|x| match x { 
		Element::Term(mut g) => Element::Term( { g.push(Element::Num(false,1,1)); g } ), 
		a => Element::Term(vec![a, Element::Num(false,1,1)])
	}) >> (g)
));

named!(pub expression <Element>, do_parse!(
  opt!(tag!("+")) >>
  first : alt_complete!(minusexpr | term | element) >>
  rest: many0!(
    alt_complete!(do_parse!(ws!(tag!("+")) >> f: alt_complete!(term | element) >> (f)) 
    | minusexpr
   )) >>
  (match rest.len() {
  	0 => first,
  	_ => Element::SubExpr({let mut a = vec![first]; a.extend(rest); a})
  	})

  	
));

named!(term <Element>, map!(separated_nonempty_list_complete!(char!('*'), element), |x| if x.len() == 1 { x[0].clone() } else { Element::Term(x) } ));
named!(variable <Element>, map!(ws!(varname),|v| Element::Var(v)));
named!(element <Element>, alt_complete!(map!(function, |x| Element::Fn(x)) | exprparen | numberdiv | numbersimple | rangedwildcard | wildcard | variable));

named!(number <(bool,u64)>, do_parse!(pos: opt!(tag!("-")) >>  val: map_res!(map_res!(ws!(digit), str::from_utf8), FromStr::from_str) >> 
    (match pos { Some(_) => false, _ => true}, val) ));

named!(numbersimple <Element>, do_parse!(
    num: number >>
    (Element::Num(num.0, num.1, 1))));

named!(numberdiv <Element>, do_parse!(
    num: number >> // make optional?
    ws!(tag!("/")) >>
    den : number >>
    (Element::Num((num.0 & den.0) || (!num.0 && !den.0) , num.1, den.1))
    )
);

named!(numorder <NumOrder>, ws!(
	alt_complete!(
	map!(tag!(">="), |_| NumOrder::GreaterEqual) |
	map!(tag!(">"), |_| NumOrder::Greater) |
	map!(tag!("=="), |_| NumOrder::Equal) |
	map!(tag!("<="), |_| NumOrder::SmallerEqual) |
	map!(tag!("<"), |_| NumOrder::Smaller)
	)));
named!(numrange <Element>, do_parse!(no: numorder >> num: alt_complete!(numberdiv | numbersimple) >> (match num {
	Element::Num(pos, num, den) => Element::NumberRange(pos, num, den, no), _ => unreachable!() })));
named!(set <Vec<Element>>, ws!(delimited!(char!('{'), separated_list!(char!(','), alt_complete!(expression | numrange)), char!('}'))));
named!(wildcard <Element>, do_parse!(name: ws!(varname) >> ws!(tag!("?")) >> r: opt!(set) >> 
    (Element::Wildcard(name, match r { Some(a) => a, None => vec![]}))));
named!(rangedwildcard <Element>, do_parse!(ws!(tag!("?")) >> name: ws!(varname) >> (Element::VariableArgument(name))));

named!(pub splitarg <Statement>, do_parse!(ws!(tag!("splitarg")) >> name: ws!(varname) >> ws!(tag!(";")) >> ( Statement::SplitArg(name) ) ) );
named!(pub print <Statement>, do_parse!(ws!(tag!("print")) >> ws!(tag!(";")) >> ( Statement::Print ) ) );
named!(pub expand <Statement>, do_parse!(ws!(tag!("expand")) >> ws!(tag!(";")) >> ( Statement::Expand ) ) );
named!(pub multiply <Statement>, do_parse!(ws!(tag!("multiply")) >> e: ws!(expression) >> ws!(tag!(";")) >> ( Statement::Multiply(e) ) ) );
named!(pub sort <String>, do_parse!(ws!(tag!("sort")) >> m: opt!(ws!(map_res!(alpha, std::str::from_utf8))) >> ws!(tag!(";")) >> (
  match m { Some(x) => x.to_owned(), None => "sort".to_owned() } ) ) );

named!(repeatblock <Statement>, do_parse!(
    ws!(tag!("repeat;")) >>
    sts : many0!(complete!(statement)) >>
    ws!(tag!("endrepeat;")) >>
   (Statement::Repeat(sts))
  )
);

named!(repeat <Statement>, do_parse!(
    ws!(tag_no_case!("repeat")) >>
    st: statement >>
   (Statement::Repeat(vec![st]))
  )
);


named!(pub idstatement <Statement>, do_parse!(
    ws!(tag!("id")) >>
    mode: ws!(map!(opt!(alt_complete!(
          map!(tag!("all"),|_|{ IdentityStatementMode::All}) | 
          map!(tag!("many"),|_|{ IdentityStatementMode::Many}))),
      |y : Option<IdentityStatementMode>| {y.unwrap_or(IdentityStatementMode::Once)})) >>
    lhs: term >>
    ws!(tag!("=")) >>
    rhs: expression >>
    ws!(tag!(";")) >>
   (Statement::IdentityStatement(IdentityStatement {mode, lhs, rhs}))
  )
);

named!(pub function <Func>, do_parse!(
  fnname : ws!(alt_complete!(builtin | varname)) >>
    args: ws!(delimited!(char!('('), separated_list!(char!(','), alt_complete!( expression | exprparen ) ), char!(')'))) >>
  (Func { name: fnname, args: args})
  )
);

named!(input <Element>, do_parse!(ws!(tag!("IN")) >> ws!(tag!("=")) >> t: expression >> ws!(tag!(";")) >> (t)));

fn eol(chr: u8) -> bool { chr != b'\n' }
named!(comment, do_parse!(ws!(tag!("//")) >> a: take_while!(eol) >> (a))); // comment ends on line end or eof

named!(blockcomment, ws!(delimited!(tag!("/*"), take_until!("*/"), tag!("*/"))));


named!(statement <Statement>, do_parse!(
    many0!(alt_complete!(blockcomment | comment)) >>
    id: alt_complete!(repeatblock | repeat | multiply| idstatement | splitarg | expand | print) >>
    (id)
  )
);

// FIXME: why so complicated?
named!(module <Module>, do_parse!(
  ids : complete!(many0!(complete!(statement))) >>
  name: sort >>
  complete!(many0!(alt_complete!(blockcomment | comment))) >>
  (Module { name: name, statements : ids }))
);

// FIXME: why so complicated?
named!(program <Program>, do_parse!(
  input: input >> 
  mods : complete!(many0!(module)) >>
  (Program { input: input, modules : mods }))
);

pub fn parse_file(filename: &str) -> Program {
  let mut f = File::open(filename).expect(&format!("Unable to open file {:?}", filename));
  let mut s  = vec![];
  f.read_to_end(&mut s).expect("Unable to read file");

  let a = program(&s);
  if let Some(ref r) = a.remaining_input() {
    if *r != [] {
      panic!("Could not parse file completely at: {}",  str::from_utf8(&r).expect("No utf-8"));
    }
  }

  a.to_result().expect("Module parsing error")
}