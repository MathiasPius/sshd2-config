use crate::Directive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope<'a> {
    Global,
    Match(&'a Directive<'a>),
}

pub struct ScopedIterator<'a, I: Iterator<Item = &'a Directive<'a>>> {
    iter: I,
    current_scope: Scope<'a>,
}

impl<'a, I: Iterator<Item = &'a Directive<'a>>> Iterator for ScopedIterator<'a, I> {
    type Item = (Scope<'a>, &'a Directive<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let directive = self.iter.next()?;

        if let Directive::MatchBlock(block) = directive {
            self.current_scope = Scope::Match(directive);

            // `Match all` is equivalent to closing an existing `Match` block.
            if block.len() == 1 {
                if let Some(block) = block.first() {
                    if block.as_ref().to_lowercase() == "all" {
                        self.current_scope = Scope::Global
                    }
                }
            }

            Some((Scope::Global, directive))
        } else {
            Some((self.current_scope, directive))
        }
    }
}

pub trait DirectivesByScope<'a>: Iterator<Item = &'a Directive<'a>> + Sized {
    fn by_scope(self) -> ScopedIterator<'a, Self> {
        ScopedIterator {
            iter: self,
            current_scope: Scope::Global,
        }
    }
}

impl<'a, I: Iterator<Item = &'a Directive<'a>>> DirectivesByScope<'a> for I {}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{
        scope::{DirectivesByScope, Scope},
        AllowAgentForwarding, AllowTcpForwarding, Config, Directive, MatchBlock,
    };

    #[test]
    pub fn build_scoped() {
        let config = indoc! {"
        AllowTcpForwarding no
        Match User backup
            AllowTcpForwarding yes
        Match Group sudo
            AllowAgentForwarding yes
        "};

        println!("{}", config);

        let config = Config::try_parse(config).unwrap();

        let scoped_directives: Vec<_> = config.directives().by_scope().collect();

        println!("{:#?}", scoped_directives);
        assert_eq!(
            vec![
                (
                    Scope::Global,
                    &Directive::AllowTcpForwarding(AllowTcpForwarding::No)
                ),
                (
                    Scope::Global,
                    &Directive::MatchBlock(vec![
                        MatchBlock::new("User"),
                        MatchBlock::new("backup")
                    ])
                ),
                (
                    Scope::Match(&Directive::MatchBlock(vec![
                        MatchBlock::new("User"),
                        MatchBlock::new("backup")
                    ])),
                    &Directive::AllowTcpForwarding(AllowTcpForwarding::Yes)
                ),
                (
                    Scope::Global,
                    &Directive::MatchBlock(vec![MatchBlock::new("Group"), MatchBlock::new("sudo")])
                ),
                (
                    Scope::Match(&Directive::MatchBlock(vec![
                        MatchBlock::new("Group"),
                        MatchBlock::new("sudo")
                    ])),
                    &Directive::AllowAgentForwarding(AllowAgentForwarding::Yes)
                )
            ],
            scoped_directives
        );
    }
}
