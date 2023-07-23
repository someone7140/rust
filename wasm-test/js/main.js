import init, { check_follow } from "../pkg/follow_check_wasm.js";
import { InputState } from "./input_state.js";

const onSubmitClick = () => {
  console.log(check_follow(state.getInput().userId, state.getInput().password));
};

const state = new InputState("#submitForm", onSubmitClick);

init().then(() => {
  state.updateLoading(false);
});
