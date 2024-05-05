use std::collections::HashMap;

use crate::module::{Module, ModuleType};

pub fn construct_module_map(text: &str) -> HashMap<String, Module> {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    let mut modules: HashMap<String, Module> = text
        .lines()
        .into_iter()
        .map(|line| {
            let components: Vec<&str> = line.split_whitespace().collect();

            let name = *components.first().unwrap();

            let id = match name {
                "broadcaster" => String::from(name),
                _ => name.chars().skip(1).collect(),
            };

            let outputs: Vec<String> = components
                .iter()
                .skip(2)
                .map(|value| value.chars().filter(|symbol| symbol != &',').collect())
                .collect();

            for output in &outputs {
                let mut target = inputs.get(output).unwrap_or(&vec![]).to_vec();
                target.push(id.to_string());

                inputs.insert(output.to_string(), target);
            }

            match name {
                _ if name.contains("&") => (id, Module::new(ModuleType::Conjunction, outputs)),
                _ if name.contains("%") => (id, Module::new(ModuleType::FlipFlop, outputs)),
                "broadcaster" => (id, Module::new(ModuleType::Broadcaster, outputs)),
                _ => (id, Module::new(ModuleType::Untyped, outputs)),
            }
        })
        .collect();

    for (id, module) in modules.iter_mut() {
        let input_modules = inputs.get(id);

        if input_modules.is_none() {
            continue;
        }

        module.set_inputs(input_modules.unwrap().to_vec());
    }

    modules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_module_map() {
        let text = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";

        let result = construct_module_map(text);

        assert_eq!(result.len(), 5);
    }
}
