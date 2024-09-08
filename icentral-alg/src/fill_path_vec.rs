crate::ix!();

pub fn fill_path_vec() -> Vec<String> {
    
    let mut vec: Vec<String> = default!();

    let path_f: &'static str = "/home/jamourft/Desktop/Research/Betweenness-Centrality/data/fj_lcc_graphs/";

    let graphs_arr: Vec<&'static str> = vec!{
        "Erdos02.lcc.net",
        "Erdos972.lcc.net",
        "Cagr.lcc.net",
        "Eva.lcc.net",
        "Epa.lcc.net",
        "Contact.lcc.net",
        "Wiki-Vote.lcc.net"
    };

    for i in 0..7 {

        let s: String = format!("{}{}", path_f, graphs_arr[i]);

        vec.push(s);
    }

    vec
}


