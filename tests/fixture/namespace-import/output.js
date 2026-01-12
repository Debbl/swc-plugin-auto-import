import * as motion from "framer-motion";
import * as _ from "lodash";
const result = _.map([
    1,
    2,
    3
], (n)=>n * 2);
const doubled = _.multiply(5, 2);
const animated = motion.div({
    animate: {
        x: 100
    }
});
