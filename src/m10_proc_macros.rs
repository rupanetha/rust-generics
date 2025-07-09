#[cfg(test)]
mod test {
  use my_proc_macro::function_to_string;
  use ai_functions::ai_function;

  const _OUTPUT: &str = "";

  #[ai_function]
  fn another_ai_function(_whatever_param: &str) {
    /// This is an awesome function, from the crates.io libraries
    /// We shall give it to an AI to guess and output
    /// In a structured manner
    println!("{}", _OUTPUT);
  }

}