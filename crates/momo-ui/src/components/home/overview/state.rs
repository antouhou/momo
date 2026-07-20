pub(super) const OVERVIEW_CARD_COUNT: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct OverviewCarouselState {
    visible_cards: [bool; OVERVIEW_CARD_COUNT],
    active_card_index: Option<usize>,
}

impl Default for OverviewCarouselState {
    fn default() -> Self {
        Self {
            visible_cards: [true; OVERVIEW_CARD_COUNT],
            active_card_index: Some(OVERVIEW_CARD_COUNT / 2),
        }
    }
}

impl OverviewCarouselState {
    pub(super) fn active_card_index(self) -> Option<usize> {
        self.active_card_index
    }

    pub(super) fn is_card_visible(self, card_index: usize) -> bool {
        self.visible_cards.get(card_index).copied().unwrap_or(false)
    }

    pub(super) fn previous_card_index(self) -> Option<usize> {
        let active_card_index = self.active_card_index?;
        (0..active_card_index)
            .rev()
            .find(|card_index| self.visible_cards[*card_index])
    }

    pub(super) fn next_card_index(self) -> Option<usize> {
        let active_card_index = self.active_card_index?;
        (active_card_index + 1..OVERVIEW_CARD_COUNT)
            .find(|card_index| self.visible_cards[*card_index])
    }

    pub(super) fn apply(&mut self, action: OverviewCarouselAction) {
        match action {
            OverviewCarouselAction::ShowPrevious => {
                if let Some(previous_card_index) = self.previous_card_index() {
                    self.active_card_index = Some(previous_card_index);
                }
            }
            OverviewCarouselAction::ShowNext => {
                if let Some(next_card_index) = self.next_card_index() {
                    self.active_card_index = Some(next_card_index);
                }
            }
            OverviewCarouselAction::CloseActive => {
                let Some(active_card_index) = self.active_card_index else {
                    return;
                };
                let next_card_index = self.next_card_index();
                let previous_card_index = self.previous_card_index();
                self.visible_cards[active_card_index] = false;
                self.active_card_index = next_card_index.or(previous_card_index);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum OverviewCarouselAction {
    ShowPrevious,
    ShowNext,
    CloseActive,
}
