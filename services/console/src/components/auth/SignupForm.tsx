import Input from "../field/kinds/Input";
import { EMAIL, USERNAME } from "./authFields";

const SignupForm = () => {
	return (
		<form
			class="box"
			onSubmit={(e) => {
				e.preventDefault();
			}}
		>
			<div class="field">
				{/* biome-ignore lint/a11y/noLabelWithoutControl: bulma form */}
				<label class="label is-medium">Name</label>
				<Input value="" valid={null} config={USERNAME} handleField={() => {}} />
			</div>
			<div class="field">
				{/* biome-ignore lint/a11y/noLabelWithoutControl: bulma form */}
				<label class="label is-medium">Email</label>
				<Input value="" valid={null} config={EMAIL} handleField={() => {}} />
			</div>
			<br />
			<div class="field">
				<p class="control">
					<button
						class="button is-primary is-fullwidth"
						type="submit"
						disabled={true}
						onMouseDown={(e) => {
							e.preventDefault();
						}}
					>
						Sign up
					</button>
				</p>
			</div>
		</form>
	);
};
export default SignupForm;
