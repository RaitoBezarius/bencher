import {
	createResource,
	For,
	Match,
	Show,
	Switch,
	type Accessor,
} from "solid-js";
import { pathname } from "../../../util/url";
import { Button } from "../../../config/types";
import type { Params } from "astro";
import Field from "../../field/Field";
import FieldKind from "../../field/kind";
import DateRange from "../../field/kinds/DateRange";

export interface Props {
	apiUrl: string;
	params: Params;
	config: TableHeaderConfig;
	start_date: Accessor<undefined | string>;
	end_date: Accessor<undefined | string>;
	search: Accessor<undefined | string>;
	archived: Accessor<undefined | string>;
	handleRefresh: () => void;
	handleStartTime: (start_time: string) => void;
	handleEndTime: (end_time: string) => void;
	handleSearch: (search: string) => void;
	handleArchived: () => void;
}

export interface TableHeaderConfig {
	title: string;
	buttons: TableButton[];
}

const TableHeader = (props: Props) => {
	const title = props.config?.title;

	return (
		<nav class="level">
			<div class="level-left">
				<div class="level-item">
					<h3 class="title is-3">{title}</h3>
				</div>
			</div>

			<div class="level-right">
				<For each={props.config?.buttons}>
					{(button) => (
						<TableHeaderButton
							apiUrl={props.apiUrl}
							params={props.params}
							start_date={props.start_date}
							end_date={props.end_date}
							search={props.search}
							archived={props.archived}
							title={title}
							button={button}
							handleRefresh={props.handleRefresh}
							handleStartTime={props.handleStartTime}
							handleEndTime={props.handleEndTime}
							handleSearch={props.handleSearch}
							handleArchived={props.handleArchived}
						/>
					)}
				</For>
			</div>
		</nav>
	);
};

interface TableButton {
	title: string;
	kind: Button;
	is_allowed?: (apiUrl: string, params: Params) => boolean;
	path: (pathname: string) => string;
}

const TableHeaderButton = (props: {
	apiUrl: string;
	params: Params;
	start_date: Accessor<undefined | string>;
	end_date: Accessor<undefined | string>;
	search: Accessor<undefined | string>;
	archived: Accessor<undefined | string>;
	title: string;
	button: TableButton;
	handleRefresh: () => void;
	handleStartTime: (start_time: string) => void;
	handleEndTime: (end_time: string) => void;
	handleSearch: (search: string) => void;
	handleArchived: () => void;
}) => {
	const [isAllowed] = createResource(props.params, (params) =>
		props.button.is_allowed?.(props.apiUrl, params),
	);

	return (
		<p class="level-item">
			<Switch>
				<Match when={props.button.kind === Button.DATE_TIME}>
					<div class="box">
						<DateRange
							start_date={props.start_date}
							end_date={props.end_date}
							handleStartTime={props.handleStartTime}
							handleEndTime={props.handleEndTime}
						/>
					</div>
				</Match>
				<Match when={props.button.kind === Button.SEARCH}>
					<Field
						kind={FieldKind.SEARCH}
						fieldKey="search"
						value={props.search() ?? ""}
						config={{
							placeholder: `Search ${props.title}`,
						}}
						handleField={(_key, search, _valid) =>
							props.handleSearch(search as string)
						}
					/>
				</Match>
				<Match when={props.button.kind === Button.ADD}>
					<Show
						when={isAllowed.loading || isAllowed()}
						fallback={
							<button
								type="button"
								class="button"
								title={`Add ${props.button.title}`}
								disabled={true}
							>
								<span class="icon">
									<i class="fas fa-plus" />
								</span>
								<span>Add</span>
							</button>
						}
					>
						<a
							class="button"
							title={`Add ${props.button.title}`}
							href={props.button.path(pathname())}
						>
							<span class="icon">
								<i class="fas fa-plus" />
							</span>
							<span>Add</span>
						</a>
					</Show>
				</Match>
				<Match when={props.button.kind === Button.INVITE}>
					<Show
						when={isAllowed()}
						fallback={
							<button
								type="button"
								class="button"
								title={`Invite to ${props.button.title}`}
								disabled={true}
							>
								<span class="icon">
									<i class="fas fa-envelope" />
								</span>
								<span>Invite</span>
							</button>
						}
					>
						<a
							class="button"
							title={`Invite to ${props.button.title}`}
							href={props.button.path(pathname())}
						>
							<span class="icon">
								<i class="fas fa-envelope" />
							</span>
							<span>Invite</span>
						</a>
					</Show>
				</Match>
				<Match when={props.button.kind === Button.REFRESH}>
					<button
						class="button"
						type="button"
						title={`Refresh ${props.title}`}
						onMouseDown={(e) => {
							e.preventDefault();
							props.handleRefresh();
						}}
					>
						<span class="icon">
							<i class="fas fa-sync-alt" />
						</span>
						<span>Refresh</span>
					</button>
				</Match>
				<Match when={props.button.kind === Button.ARCHIVED}>
					<button
						class={`button${props.archived() === "true" ? " is-primary" : ""}`}
						type="button"
						title={`Toggle archived ${props.title}`}
						onMouseDown={(e) => {
							e.preventDefault();
							props.handleArchived();
						}}
					>
						<span class="icon">
							<i class="fas fa-archive" />
						</span>
						<span>Archived</span>
					</button>
				</Match>
			</Switch>
		</p>
	);
};

export default TableHeader;
