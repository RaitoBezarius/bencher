enum Collection {
	// Legal
	legal = "legal",
	// Docs
	tutorial = "tutorial",
	how_to = "how-to",
	explanation = "explanation",
	reference = "reference",
	// API
	organizations = "organizations",
	projects = "projects",
	users = "users",
	server = "server",
	// Learn
	benchmarking_rust = "benchmarking-rust",
	case_study = "case-study",
	engineering = "engineering",
	// Onboard,
	onboard = "onboard",
}

export const ApiCollections = [Collection.organizations];

export const collectionPath = (collection: Collection) => {
	switch (collection) {
		case Collection.legal:
			return "legal";
		case Collection.tutorial:
			return "tutorial";
		case Collection.how_to:
			return "how-to";
		case Collection.explanation:
			return "explanation";
		case Collection.reference:
			return "reference";
		case Collection.organizations:
			return "organizations";
		case Collection.projects:
			return "projects";
		case Collection.users:
			return "users";
		case Collection.server:
			return "server";
		case Collection.benchmarking_rust:
			return "benchmarking/rust";
		case Collection.case_study:
			return "case-study";
		case Collection.engineering:
			return "engineering";
		case Collection.onboard:
			return "onboard";
	}
};

export default Collection;
