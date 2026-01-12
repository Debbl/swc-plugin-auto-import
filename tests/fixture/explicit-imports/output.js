import * as motion from "framer-motion";
import _ from "lodash";
import { useState as useSignal } from "react";
import { computed, ref } from "vue";
const count = ref(0);
const doubled = computed(()=>count.value * 2);
const [value, setValue] = useSignal(10);
const result = _.map([
    1,
    2,
    3
], (n)=>n * 2);
const animated = motion.div();
