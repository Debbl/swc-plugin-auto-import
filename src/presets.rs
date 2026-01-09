use std::collections::HashMap;

/// Preset configuration - returns mapping from package name to import list
/// Each import is a (name, alias) tuple, where alias is None means no alias
pub fn get_preset_imports(preset: &str) -> HashMap<String, Vec<(String, Option<String>)>> {
    let mut map = HashMap::new();

    match preset {
        "vue" => {
            map.insert(
                "vue".to_string(),
                vec![
                    ("ref".to_string(), None),
                    ("computed".to_string(), None),
                    ("reactive".to_string(), None),
                    ("watch".to_string(), None),
                    ("watchEffect".to_string(), None),
                    ("onMounted".to_string(), None),
                    ("onUnmounted".to_string(), None),
                    ("onBeforeMount".to_string(), None),
                    ("onBeforeUnmount".to_string(), None),
                    ("onUpdated".to_string(), None),
                    ("onBeforeUpdate".to_string(), None),
                    ("nextTick".to_string(), None),
                    ("defineComponent".to_string(), None),
                    ("createApp".to_string(), None),
                    ("toRef".to_string(), None),
                    ("toRefs".to_string(), None),
                    ("unref".to_string(), None),
                    ("isRef".to_string(), None),
                ],
            );
        }
        "react" => {
            map.insert(
                "react".to_string(),
                vec![
                    // Common React API
                    ("useState".to_string(), None),
                    ("useCallback".to_string(), None),
                    ("useMemo".to_string(), None),
                    ("useEffect".to_string(), None),
                    ("useRef".to_string(), None),
                    ("useContext".to_string(), None),
                    ("useReducer".to_string(), None),
                    ("useImperativeHandle".to_string(), None),
                    ("useDebugValue".to_string(), None),
                    ("useDeferredValue".to_string(), None),
                    ("useLayoutEffect".to_string(), None),
                    ("useTransition".to_string(), None),
                    ("startTransition".to_string(), None),
                    ("useSyncExternalStore".to_string(), None),
                    ("useInsertionEffect".to_string(), None),
                    ("useId".to_string(), None),
                    ("lazy".to_string(), None),
                    ("memo".to_string(), None),
                    ("createRef".to_string(), None),
                    ("forwardRef".to_string(), None),
                    // Additional React API
                    ("cache".to_string(), None),
                    ("cacheSignal".to_string(), None),
                    ("createContext".to_string(), None),
                    ("use".to_string(), None),
                    ("useOptimistic".to_string(), None),
                    ("useEffectEvent".to_string(), None),
                    ("useActionState".to_string(), None),
                    ("Fragment".to_string(), None),
                    ("Suspense".to_string(), None),
                    ("Activity".to_string(), None),
                ],
            );
        }
        "react-dom" => {
            map.insert(
                "react-dom".to_string(),
                vec![
                    ("useFormStatus".to_string(), None),
                    ("createPortal".to_string(), None),
                    ("flushSync".to_string(), None),
                    ("preconnect".to_string(), None),
                    ("prefetchDNS".to_string(), None),
                    ("preinit".to_string(), None),
                    ("preinitModule".to_string(), None),
                    ("preload".to_string(), None),
                    ("preloadModule".to_string(), None),
                ],
            );
        }
        "vue-router" => {
            map.insert(
                "vue-router".to_string(),
                vec![
                    ("useRouter".to_string(), None),
                    ("useRoute".to_string(), None),
                ],
            );
        }
        "react-router" => {
            map.insert(
                "react-router-dom".to_string(),
                vec![
                    ("useNavigate".to_string(), None),
                    ("useLocation".to_string(), None),
                    ("useParams".to_string(), None),
                    ("useSearchParams".to_string(), None),
                ],
            );
        }
        _ => {}
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vue_preset() {
        let imports = get_preset_imports("vue");
        assert!(imports.contains_key("vue"));
        let vue_imports = imports.get("vue").unwrap();
        assert!(vue_imports.len() >= 17);
        assert!(vue_imports.contains(&("ref".to_string(), None)));
        assert!(vue_imports.contains(&("computed".to_string(), None)));
    }

    #[test]
    fn test_react_preset() {
        let imports = get_preset_imports("react");
        assert!(imports.contains_key("react"));
        let react_imports = imports.get("react").unwrap();
        assert!(react_imports.len() >= 30);
        assert!(react_imports.contains(&("useState".to_string(), None)));
        assert!(react_imports.contains(&("useEffect".to_string(), None)));
        assert!(react_imports.contains(&("lazy".to_string(), None)));
        assert!(react_imports.contains(&("memo".to_string(), None)));
    }

    #[test]
    fn test_react_dom_preset() {
        let imports = get_preset_imports("react-dom");
        assert!(imports.contains_key("react-dom"));
        let react_dom_imports = imports.get("react-dom").unwrap();
        assert_eq!(react_dom_imports.len(), 9);
        assert!(react_dom_imports.contains(&("useFormStatus".to_string(), None)));
        assert!(react_dom_imports.contains(&("createPortal".to_string(), None)));
    }

    #[test]
    fn test_router_presets() {
        let vue_router = get_preset_imports("vue-router");
        assert!(vue_router.contains_key("vue-router"));

        let react_router = get_preset_imports("react-router");
        assert!(react_router.contains_key("react-router-dom"));
    }
}
