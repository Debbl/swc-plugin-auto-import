import { useFetch as useMyFetch, useMouse } from "@vueuse/core";
import axios from "axios";
import * as _ from "lodash";
const { x, y } = useMouse();
const data = useMyFetch('/api/users');
const response = axios.get('/api/data');
const result = _.map([
    1,
    2,
    3
], (n)=>n * 2);
