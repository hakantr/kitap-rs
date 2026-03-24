import os
import re

# List of words/identifiers to remove from parentheses
to_remove = [
    "search", "contents", "performance", "query", "file_path", "args", "line", "results",
    "ignore_case", "giveaway", "user_preference", "store", "shoe_size", "Config", "Config::build",
    "add", "add_two", "internal_adder", "larger_can_hold_smaller", "smaller_cannot_hold_larger",
    "it_works", "exploration", "another", "Rectangle", "width", "height", "can_hold", "it_adds_two",
    "greeting", "name", "greeting_contains_name", "greater_than_100", "Guess", "value",
    "read_username_from_file", "username_file_result", "username_file", "username",
    "last_char_of_first_line", "text", "ip", "home", "first_sentence", "novel", "part",
    "ImportantExcerpt", "first_word", "bytes", "item", "my_string", "word", "my_string_literal",
    "level", "announce_and_return_part", "announcement", "longest_with_an_announcement", "ann",
    "longest", "result", "string1", "string2", "Pair", "cmp_display", "returns_summarizable",
    "switch", "some_function", "notify", "summarize_author", "repost", "reply", "content",
    "author", "location", "headline", "NewsArticle", "SocialPost", "summarize", "Summary",
    "mixup", "other", "distance_from_origin", "wont_work", "integer", "float", "Point",
    "largest_char", "largest_i32", "list", "number", "largest", "number_list", "SpreadsheetCell",
    "Int", "Float", "Text", "row", "does_not_exist", "first", "third", "v",
    "search_case_insensitive", "ShirtColor", "Red", "Blue", "Inventory", "shirts",
    "most_stocked", "user_pref1", "user_pref2", "add_one_v1", "add_one_v2", "add_one_v3",
    "add_one_v4", "example_closure", "only_borrows", "borrows_mutably", "sort_operations",
    "num_sort_operations", "v1", "v1_iter", "val", "total", "v2", "Shoe", "size", "style",
    "shoes_in_size", "shoes", "filters_by_size", "sneaker", "sandal", "boot",
    "iterator_demonstration", "iterator_sum",
    "functionality", "method signatures", "metadata", "return value", "implementation",
    "elements", "evaluate", "implement", "reference", "references", "string slice", "string slices",
    "borrowing", "ownership", "scope", "scopes", "mutable", "immutable", "pattern matching",
    "generics", "type annotation", "compiler error", "compile time", "runtime", "trait object",
    "wrapper", "instance", "instances", "signature", "signatures", "associated types", "concrete types",
    "explicit", "documentation", "prelude", "iterator", "iterators", "profile", "fail", "pass",
    "fails", "passes", "called", "calling", "call", "thread", "threads", "panic", "panics",
    "panicked", "debug", "debugging", "framework", "primitive types", "equality", "custom",
    "required", "placeholders", "documenting", "requirements", "exact equality", "exclude",
    "actual value", "type", "types", "test harness", "substring", "unique", "precise", "bodies",
    "pinpoint", "in isolation", "private interfaces", "external", "compel", "test coverage",
    "directory structure", "subdirectories", "noticeable", "naming convention", "expose",
    "testing features", "private implementation details", "logic bugs", "command line tool",
    "use case", "file path", "configure", "environment variable", "redirect", "real-world",
    "background", "closures", "process", "capability", "nested", "parent", "ambiguous", "invalid",
    "annotate", "infer", "execution", "invoked", "alias", "variables", "provides no arguments",
    "erroneous", "error handling", "ignore", "repeated words", "handle", "flaws", "refactor",
    "practice", "configuration", "logic", "structure", "permission", "regardless", "maintainers",
    "end users", "refactoring", "responsibility", "allocating", "organizational", "binary projects",
    "concerns", "command line parsing logic", "limited", "setting up", "separating concerns",
    "verify its correctness", "rework", "extract", "collecting", "overkill", "incremental",
    "argument parsing", "identify the cause", "tuple", "immediately", "abstraction", "implies",
    "conveying", "meaningful", "owned", "owner", "borrow", "borrowing rules", "own", "lifetimes",
    "straightforward", "trade-off", "runtime cost", "ownership problems", "avoid", "tendency",
    "okay", "hyperoptimize", "experienced", "acceptable", "struct field", "idiomatic", "associates",
    "block", "fixing", "intended for", "verify", "length", "condition", "end", "reasonable",
    "extraneous", "usage", "indicates", "successful", "error", "describe", "signal", "practical",
    "string literal", "error values", "doesn't pass enough arguments", "wrapped", "error case",
    "exit", "user-friendly", "nonzero error code", "implement by hand", "exit status", "error state",
    "convention", "wrapping", "inner value", "closure", "anonymous", "vertical pipes", "static",
    "exit status code", "handling", "no longer get", "output", "friendlier", "configuration parsing",
    "concise", "verify by inspection", "program logic", "consolidate", "body", "return type",
    "unit type", "returned value", "implements", "particular", "flexibility", "keyword", "dynamic",
    "in favor of", "current function", "caller", "to handle", "success cases", "wrap", "using",
    "side effects", "might indicate", "indicate", "checking to see", "probably", "meant", "rectify",
    "slight difference", "detecting", "in both cases", "the same", "responsibilities",
    "searching function", "binary", "contexts", "define", "public", "designate", "binary crate",
    "bring into the scope", "bring into scope", "printing out", "if no errors occur", "printing",
    "happens", "returns", "not printed as they're found", "set ourselves up", "modular", "old",
    "new", "advantage", "write some tests", "core", "searching logic", "code design", "high test coverage",
    "test-drive", "backslash", "newline character", "assert", "principles", "connected", "slices",
    "assumes", "safety checking", "lifetime annotations", "help text", "lifetime syntax", "iterate",
    "iterating", "matching", "mini version", "command line parsing", "round out", "case-insensitive",
    "set", "case-sensitive", "shouldn't match", "casing", "fail to compile", "skeleton implementation",
    "input", "lowercase", "shadowing", "treat", "basic Unicode", "creates new data", "allocate",
    "matches", "configuration option", "initializing", "set to any value", "not set", "unset",
    "persist", "uppercase letters", "options", "manage", "takes precedence", "case sensitivity",
    "dealing with", "stdout", "stderr", "intentionally", "redirecting", "stream", "command line programs",
    "well behaved", "shell", "well tested", "reproduction", "inefficient", "doesn't own",
    "iterator functionality", "access", "allocation", "trait bounds", "generic type", "specification",
    "reproduced", "iterator adapter methods", "minimize", "enhancement", "concurrent access",
    "contain", "improvement", "iterator adapter", "tests", "difference in behavior", "laziness",
    "style", "bits", "high-level objective", "abstracts away", "filtering condition", "equivalent",
    "intuitive", "lower-level", "benchmark", "compare", "comprehensive", "variations", "high-level",
    "roughly", "compiled down", "zero-cost abstractions", "runtime overhead", "imposes", "analogous",
    "keynote", "zero-overhead", "In general", "zero-overhead principle", "by hand", "loop unrolling",
    "array access", "bounds checking", "eliminating", "optimizations", "resultant code", "efficient",
    "higher level", "runtime performance penalty", "functional programming language", "inspired",
    "contribute", "runtime performance", "not affected", "strive", "goal", "expressiveness", "share",
    "bugs", "correctness", "type system", "automated software tests", "type checking", "borrow checking",
    "testing facilities", "annotations", "unit tests", "integration tests", "test runner binary",
    "template", "test runner", "test failure", "expression", "derivable traits", "environments",
    "environment"
]

patterns = [r' \(' + re.escape(p) + r'\)' for p in to_remove]

for root, dirs, files in os.walk('src'):
    for file in files:
        if file.endswith('.md'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            new_content = content
            for p in patterns:
                new_content = re.sub(p, '', new_content)
            
            if new_content != content:
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Updated {path}")
