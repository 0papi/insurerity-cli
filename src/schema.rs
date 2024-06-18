use std::collections::HashMap;
pub const NEXT_JS_ADDONS: [&str; 4] = [
    "Use Tanstack (React) Query for data fetching",
    "Use TailwindCSS for styling",
    "Use Typescript",
    "use Zustand for state management",
];

pub fn create_addons_map() -> HashMap<&'static str, &'static str> {
    let mut addons_map = HashMap::new();
    addons_map.insert(NEXT_JS_ADDONS[0], "react-query");
    addons_map.insert(NEXT_JS_ADDONS[1], "tailwindcss");
    addons_map.insert(NEXT_JS_ADDONS[2], "typescript");
    addons_map.insert(NEXT_JS_ADDONS[3], "zustand");
    addons_map
}

pub const AUTH_OPTIONS: [&str; 2] = ["Next-auth", "Firebase"];

pub const PACKAGE_MANAGERS: [&str; 3] = ["npm", "yarn", "pnpm"];
