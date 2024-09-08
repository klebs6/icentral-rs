crate::ix!();

/*
 | When Jesus came into the coasts of Cæsarea
 | Philippi, he asked his disciples, saying, Whom do
 | men say that I, the Son of man, am?
 |
 | And they said, Some say that thou art John the
 | Baptist: some, Elias; and others, Jeremias, or one
 | of the prophets.
 |
 | He saith unto them, But whom say ye that I am?
 |
 | And Simon Peter answered and said, Thou art the
 | Christ, the Son of the living God.
 |
 | And Jesus answered and said unto him, Blessed art
 | thou, Simon Bar-jona: for flesh and blood hath not
 | revealed it unto thee, but my Father which is in
 | heaven.
 |
 | And I say also unto thee, That thou art Peter, and
 | upon this rock I will build my church; and the
 | gates of hell shall not prevail against it.
 |
 | And I will give unto thee the keys of the kingdom
 | of heaven: and whatsoever thou shalt bind on earth
 | shall be bound in heaven: and whatsoever thou
 | shalt loose on earth shall be loosed in heaven.
 |
 | Then charged he his disciples that they should
 | tell no man that he was Jesus the Christ.
 |
 | From that time forth began Jesus to shew unto his
 | disciples, how that he must go unto Jerusalem, and
 | suffer many things of the elders and chief priests
 | and scribes, and be killed, and be raised again
 | the third day.
 |
 | Then Peter took him, and began to rebuke him,
 | saying, Be it far from thee, Lord: this shall not
 | be unto thee.
 |
 | But he turned, and said unto Peter, Get thee
 | behind me, Satan: thou art an offence unto me: for
 | thou savourest not the things that be of God, but
 | those that be of men.
 |
 | Then said Jesus unto his disciples, If any man
 | will come after me, let him deny himself, and take
 | up his cross, and follow me.
 |
 | For whosoever will save his life shall lose it:
 | and whosoever will lose his life for my sake shall
 | find it.
 |
 | For what is a man profited, if he shall gain the
 | whole world, and lose his own soul? or what shall
 | a man give in exchange for his soul?
 |
 | For the Son of man shall come in the glory of his
 | Father with his angels; and then he shall reward
 | every man according to his works.
 |
 | Verily I say unto you, There be some standing
 | here, which shall not taste of death, till they
 | see the Son of man coming in his kingdom.
 |
 | - The Book of Matthew, Ch. 16
 */
impl fmt::Debug for SubGraph {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding = f.debug_struct("SubGraph");

        let mut builder = binding;

        builder.field("nodes_map",             &self.nodes_map);
        builder.field("edges",                 &self.edges);
        builder.field("label_map",             &self.label_map);
        builder.field("parents",               &self.parents);

        builder.field("path_counts",           &self.path_counts);
        builder.field("new_path_counts",       &self.new_path_counts);
        builder.field("inc_path_counts",       &self.inc_path_counts);

        builder.field("distances",             &self.distances);

        builder.field("pair_dependencies",     &self.pair_dependencies);
        builder.field("new_pair_dependencies", &self.new_pair_dependencies);

        builder.field("sigmas",                &self.sigmas);
        builder.field("new_sigmas",            &self.new_sigmas);

        builder.field("visit_markers",         &self.visit_markers);
        builder.field("stack",                 &self.stack);
        builder.field("queue",                 &self.queue);

        builder.finish()
    }
}

pub struct SubGraphDebugger<'g> {
    host:       &'g SubGraph,
    with_nodes: bool,
}

impl SubGraph {

    pub fn debug_without_nodes<'g>(&'g self) -> SubGraphDebugger<'g> {

        SubGraphDebugger {
            host:       self,
            with_nodes: false,
        }
    }

    pub fn debug_with_nodes<'g>(&'g self) -> SubGraphDebugger<'g> {

        SubGraphDebugger {
            host:       self,
            with_nodes: true,
        }
    }
}

impl<'g> fmt::Debug for SubGraphDebugger<'g> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding = f.debug_struct("SubGraph");

        let mut builder = binding;

        builder.field(
            "edges", 
            self.host.edges()
        );

        if self.with_nodes {

            builder.field("nodes", self.host.nodes());
        }

        builder.finish()
    }
}
