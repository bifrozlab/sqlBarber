pub fn ts_kind_hash(s: &str) -> u32 {
    s.bytes()
    .map(Into::<u32>::into)
    .enumerate()
    .fold(${random_seed}u32, |hash, (i, c)| hash ^ (c * SEEDS[i%$n_seed]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_outputs_correct_hash_table_ref() {
        let hash = ts_kind_hash("table_ref");
        assert_eq!(hash, TABLE_REF);
    }

    #[test]
    fn it_outputs_correct_hash_from_list() {
        let hash = ts_kind_hash("from_list");
        assert_eq!(hash, FROM_LIST);
    }

    #[test]
    fn it_outputs_correct_hash_func_expr() {
        let hash = ts_kind_hash("func_expr");
        assert_eq!(hash, FUNC_EXPR);
    }

    #[test]
    fn it_outputs_correct_hash_identifier() {
        let hash = ts_kind_hash("identifier");
        assert_eq!(hash, IDENTIFIER);
    }

    #[test]
    fn it_outputs_correct_hash_multistmt() {
        let hash = ts_kind_hash("multistmt");
        assert_eq!(hash, MULTISTMT);
    }
}
