use structure::{Module,Statement,Element,Func,StatementResult,IdentityStatement,Program};
use id::MatchIterator;
use std::mem;

impl Element {
	fn expand(&self) -> Element {
		match self {
			&Element::Fn(Func{ref name, ref args}) => Element::Fn(Func{ name: name.clone(),
				args: args.iter().map(|x| x.expand()).collect()}),
			&Element::Term(ref fs) => {
				let mut r : Vec<Vec<Element>> = vec![vec![]];

				for f in fs {
					match f {
						&Element::SubExpr(ref s) => {
							// use cartesian product function?
							r = r.iter().flat_map(|x| s.iter().map(|y| { 
								let mut k = x.clone(); k.push(y.expand()); k } ).collect::<Vec<_>>() ).collect();
						},
						_ => {
							for rr in r.iter_mut() {
								rr.push(f.expand());
							}
						}
					}
				}

				// FIXME: this should not happen for the ground level
				Element::SubExpr(r.iter().map(|x| Element::Term(x.clone())).collect()).normalize()
			},
			&Element::SubExpr(ref f) => Element::SubExpr( f.iter().map(|x| x.expand()).collect()).normalize(),
			_ => self.clone()
		}
	}
}

#[derive(Debug)]
enum StatementIter<'a> {
	IdentityStatement(MatchIterator<'a>),
	Multiple(Vec<Element>, bool),
	Simple(Element, bool), // yield a term once
	None
}

impl<'a> StatementIter<'a> {
	fn next(&mut self) -> StatementResult<Element> {
		match *self {
			StatementIter::IdentityStatement(ref mut id) => id.next(),
			StatementIter::Multiple(ref mut f, m) => {
				if f.len() == 0 { return StatementResult::Done; }
				if m {
					StatementResult::Executed(f.pop().unwrap()) // FIXME: pops the last term
				} else {
					StatementResult::NotExecuted(f.pop().unwrap())
				}
            },
			StatementIter::Simple(..) => {
				let mut to_swap = StatementIter::None;
                mem::swap(self, &mut to_swap); //f switch self to none
                match to_swap {
                    StatementIter::Simple(f, true)  => StatementResult::Executed(f), // set the default to not executed!
                    StatementIter::Simple(f, false)  => StatementResult::NotExecuted(f), // set the default to not executed!
                    _   => panic!(), // never reached
                }
            },
			StatementIter::None => StatementResult::Done
		}
	}
}

impl Statement {
	fn to_iter<'a>(&'a self, input: &'a Element) -> StatementIter<'a> {
		match *self {
	      Statement::IdentityStatement (ref id) => {
	        StatementIter::IdentityStatement(id.to_iter(input))
	      },
	      Statement::SplitArg(ref name) => {
	        // split function arguments at the ground level
	        let subs = | n : &String , a: &Vec<Element> |  Element::Fn( Func {name: n.clone(), args: 
	              a.iter().flat_map( |x| match x { &Element::SubExpr(ref y) => y.clone(), _ => vec![x.clone()] } ).collect()});

	        match *input {
	          // FIXME: check if the splitarg actually executed!
	          Element::Fn(Func{name: ref n, args: ref a}) if *n == *name => StatementIter::Simple(subs(n, a), false),
	          Element::Term(ref fs) => {
	            StatementIter::Simple(Element::Term(fs.iter().map(|f| match f {
	              &Element::Fn(Func{name: ref n, args: ref a}) if *n == *name => subs(n, a),
	              _ => f.clone()
	            } ).collect()), false)
	          }
	          _ => StatementIter::Simple(input.clone(), false)
	        }
	      },
	      Statement::Expand => {
	      	// FIXME: treat ground level differently in the expand routine
			// don't generate all terms in one go
			match input.expand() {
				Element::SubExpr(f) => {
					if f.len() == 1 {
						 StatementIter::Simple(f[0].clone(), false)
					} else {
						 StatementIter::Multiple(f, true)
					}
				}
				a => StatementIter::Simple(a, false)
			}

	      },
	      Statement::Print => {
	      	println!("\t+{}", input);
	      	StatementIter::Simple(input.clone(), false)
	      },
	      Statement::Multiply(ref x) => {
	      	let res = match (input, x) {
	      		(&Element::Term(ref xx), &Element::Term(ref yy)) => { let mut r = xx.clone(); r.extend(yy.clone()); Element::Term(r) },
				(&Element::Term(ref xx), _) => { let mut r = xx.clone(); r.push(x.clone()); Element::Term(r) },
				(_, &Element::Term(ref xx)) => { let mut r = xx.clone(); r.push(input.clone()); Element::Term(r) },
	      		_ => Element::Term(vec![input.clone(), x.clone()])
	      	}.normalize();
	      	StatementIter::Simple(res, true)
	      },
	      _ => unreachable!()
	    }
	}
}

fn do_module_rec(input: &Element, statements: &[Statement], current_index: usize, term_affected: &mut Vec<bool>,
	output: &mut Vec<Element>) {
	if current_index == statements.len() {
		output.push(input.clone());
		return;
	}

	// handle control flow instructions
	match statements[current_index] {
		Statement::PushChange => {
			term_affected.push(false);
			return do_module_rec(input, statements, current_index + 1, term_affected, output)
		},
		Statement::JumpIfChanged(i) => { // the i should be one after the PushChange
			let l = term_affected.len();
			let repeat = term_affected[l - 1];

			if repeat {
				term_affected[l - 2] = true;
				term_affected[l - 1] = false;
				return do_module_rec(input, statements, i, term_affected, output);
			} else {
				term_affected.pop();
				return do_module_rec(input, statements, current_index + 1, term_affected, output);
			}
		},
		_ => {}
	}
	
	let mut it = statements[current_index].to_iter(&input);
	loop {
		match it.next() { // for every term
			StatementResult::Executed(ref f) => { 
				// make a copy that will be modified further in the recursion. FIXME: is this the only way?
				let mut newtermaff = term_affected.clone();
				*newtermaff.last_mut().unwrap() = true; 
				do_module_rec(f, statements, current_index + 1, &mut newtermaff, output);
			},
			StatementResult::NotExecuted(ref f) => do_module_rec(f, statements, current_index + 1, term_affected, output),
			StatementResult::Done => { return; }
		};
	}
}

impl Module {
	// flatten the statement structure and use conditional jumps
	/*fn to_control_flow(statements: &[Statement], start_index: usize) -> Vec<Statement> {
		let mut newstat = vec![];
		let mut newindex = start_index;
		for x in statements.iter() {
			match x {
				&Statement::Repeat(ref ss) => {
					newstat.push(Statement::PushChange);
					newstat.extend(to_control_flow(ss, newindex + 1));
					newstat.push(Statement::JumpIfChanged(newindex + 1));
				},
				a => newstat.push(a.clone())
			}
			newindex = start_index + newstat.len();
		} 
		newstat
	}*/

	// flatten the statement structure and use conditional jumps
	fn to_control_flow_stat(statements: &[Statement], output: &mut Vec<Statement>) {
		for x in statements.iter() {
			match x {
				&Statement::Repeat(ref ss) => {
					output.push(Statement::PushChange);
					let pos = output.len();
					Module::to_control_flow_stat(ss, output);
					output.push(Statement::JumpIfChanged(pos));
				},
				a => output.push(a.clone())
			}
		}
	}

	// normalize all expressions in statements
	fn normalize_module(&mut self) {
		let oldstat = self.statements.clone();
		self.statements.clear();
		Module::to_control_flow_stat(&oldstat, &mut self.statements);

		for x in self.statements.iter_mut() {
			match *x {
				Statement::IdentityStatement(IdentityStatement{ref mut lhs, ..}) => 
					*lhs = lhs.normalize(),
					_ => {}
			}
		}
	}
}

// execute the module
pub fn do_program(program : &mut Program) {
	program.input = program.input.normalize();

	for module in program.modules.iter_mut() {
		module.normalize_module();
		println!("{}", module);

		let mut executed = vec![false];
		let mut output = vec![];

		let inpcount;
		match program.input {
	  		Element::SubExpr(ref f) => {
	  			inpcount = f.len();
	  			// TODO: paralellize
	  			for x in f.iter() {
	  				do_module_rec(x, &module.statements, 0, &mut executed, &mut output);
	  			}

	  		},
	  		ref f => { inpcount = 1; do_module_rec(f, &module.statements, 0, &mut executed, &mut output); }
	  	}

	  	let genterms = output.len();
	  	program.input = Element::SubExpr(output).normalize();
	  	let outterms = match program.input {
	  		Element::SubExpr(ref x) => x.len(),
	  		_ => 1
	  	};

        println!("{} -- \t terms in: {}\tgenerated: {}\tterms out: {}", module.name,
            	inpcount, genterms, outterms);

        // display the terms
        println!("{}", program.input);
	}
}

// execute the module
/*
pub fn do_module_it(module : &Module) {
  let mut curterm = Element::Num(0);// FIXME

  let mut iterators = (0..module.statements.len() + 1).map(|_| StatementIter::None).collect::<Vec<_>>();
  iterators[0] = StatementIter::Multiple(
  	match module.input {
  		Element::SubExpr(ref f) => f.clone(),
  		ref f => vec![f.clone()]
  	}
  ); // first entry is all the terms


  // loops over all terms
  // TODO: paralellize
	'next: loop {
		let mut i = iterators.len() - 1;
		if let Some(x) = iterators[i].next() {
			curterm = x;

            // now update all elements after
            let mut j = i + 1;
            while j < iterators.len() {
                // create a new iterator at j based on the previous match dictionary and slice
                iterators[j] = module.statements[j - 1].to_iter(&curterm); // note: element 0 was all the terms

                match iterators[j].next() {
                    Some(y) => { }, // do nothing?
                    None => { i = j - 1; continue 'next; } // try the next match at j-1
                }

                j += 1;
            }
		}

		if i == 0 { break; } // done!
		i -= 1;
	}
}
*/