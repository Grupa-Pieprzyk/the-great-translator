<script lang="ts">
	import { Home, Settings, Sliders, Smile, Gift, ShoppingCart, AlignRight, Play } from './icons';
	import { selectedProfile } from '@/auth/auth-store';
	import MinecraftButton from '@/components/atoms/buttons/MinecraftButton.svelte';
	import IconWithText from '@/components/atoms/buttons/IconWithText.svelte';
	import { autoUpdaterStore } from '@/auto-update';
	import { Command } from '@/tauri-types';
	import { last } from 'lodash-es';
	import { launchParameters } from '@/user-settings-store';
	import { launchState } from '@/launch-state-store';
	import { userSettings } from '@/user-settings-store';
	import { DEBUG } from '@/config';

	import { HOME, SETTINGS, ABOUT_US, PROFILE, COSMETICS, STORE } from '@/router';

	import { _ } from 'svelte-i18n';

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
</script>

<div id="TheSidebar">
	<div
		class="menu-item play l-500 k-hover"
		class:active={$page.path === HOME}
		on:click={() => goto(HOME)}
	>
		<div class="icon">
			<Home />
		</div>
		<div class="text">
			{$_('sidebar_buttons.home_page')}
		</div>
	</div>

	<div
		class="menu-item play l-500 k-hover"
		class:active={$page.path === SETTINGS}
		on:click={() => goto(SETTINGS)}
	>
		<div class="icon">
			<Sliders />
		</div>
		<div class="text">
			{$_('sidebar_buttons.settings')}
		</div>
	</div>

	<div
		class="menu-item coming-soon"
		class:active={$page.path === ABOUT_US}
		on:click={() => goto(ABOUT_US)}
	>
		<div class="icon">
			<AlignRight />
		</div>
		<div class="text">
			{$_('sidebar_buttons.about_us')}
		</div>
	</div>

	<div
		class="menu-item coming-soon"
		class:active={$page.path === PROFILE}
		on:click={() => goto(PROFILE)}
	>
		<div class="icon">
			<Smile />
		</div>
		<div class="text">
			{$_('sidebar_buttons.profile')}
		</div>
	</div>

	<div class="menu-item " class:active={$page.path === COSMETICS} on:click={() => goto(COSMETICS)}>
		<div class="icon">
			<Gift />
		</div>
		<div class="text">
			{$_('sidebar_buttons.cosmetics')}
		</div>
	</div>

	<div class="menu-item " class:active={$page.path === STORE} on:click={() => goto(STORE)}>
		<div class="icon">
			<ShoppingCart />
		</div>
		<div class="text">
			{$_('sidebar_buttons.store')}
		</div>
	</div>

	{#if $selectedProfile != null}
		<div class="play-button" class:updateProgress={$autoUpdaterStore.updateProgress !== null}>
			{#if $autoUpdaterStore.updateProgress === null}
				{#if $launchState.kind === 'Exited'}
					<MinecraftButton
						on:click={() => {
							// console.log($selectedProfile.raw);
							Command.startMinecraft(
								$selectedProfile.raw,
								$userSettings.selected_version,
								$launchParameters
							);
						}}
					>
						<IconWithText>
							<Play slot="icon" />
							<h3 slot="text">
								{$_('sidebar_buttons.play_now')}
							</h3>
						</IconWithText>
					</MinecraftButton>
				{:else}
					<MinecraftButton disabled>
						<IconWithText>
							<Play slot="icon" />
							<h3 slot="text">
								{$_($launchState.kind)}
							</h3>
						</IconWithText>
					</MinecraftButton>
				{/if}
			{:else}
				<MinecraftButton>
					<IconWithText>
						<Play slot="icon" />
						<h3 slot="text">
							{$autoUpdaterStore.updateProgress}
						</h3>
					</IconWithText>
				</MinecraftButton>
			{/if}
		</div>
	{/if}
</div>

<style lang="scss">
	@mixin transform-with-padding($padding) {
		padding-top: $padding;
		/* transform: translateY(#{-$padding * 0.3}); */
	}
	#TheSidebar {
		box-sizing: border-box;
		@include grid-center-padding(2rem, 0);
		@include section-border(right);
		border-bottom: none;
		grid-template-rows: repeat(#{$menu-items-count}, 1fr) 2fr;
		.menu-item {
			@include text-hoverable-behaviour;
			@include grid-center-padding(0, $icon-standard-size * 1.3);

			grid-template-columns: 4rem 1fr;
			grid-gap: $icon-standard-size * 0.22;
			transition: all 0.1s ease-in-out;
			background-color: rgba($accent-orange-color, 0);
			border-right: 0 solid rgba(black, 0);

			& * {
				transition: all 0.2s ease-in-out;
			}

			.text {
				text-transform: capitalize;
				text-align: start;
				width: 100%;
				border-top: none;
				border-left: none;
				border-right: none;
				font-size: $sidebar-text-size;
				white-space: nowrap;
				font-weight: 700;
			}

			&:hover {
				border-right: $border-width solid $text-selected;
			}
			&.active {
				color: $accent-orange-color;
				border-right: $border-width * 2 solid $accent-orange-color;
				background-color: rgba($accent-orange-color, 0.1);

				@include highlight($accent-orange-color);
				.text {
					color: $accent-orange-color;
					font-size: $sidebar-text-size * 1.07;
				}
				:global(.icon > svg) {
					color: $accent-orange-color;
					filter: drop-shadow(0px 0px 5px $accent-orange-color);
				}
			}
		}

		.play-button {
			@include grid-center;
			text-transform: uppercase;
			align-items: end;
			&.updateProgress {
				:global(.MinecraftButton) {
					--color: gray;
					:global(.text > h3) {
						text-align: center;
						font-size: 0.4rem !important;
					}
				}
			}
			:global(.MinecraftButton) {
				--color: #{$play-button-color};
			}
			:global(.icon) {
				transform: translateY(-5%);
			}
			:global(.text) {
				transform: translateY(-5%);
			}
		}
	}
</style>
