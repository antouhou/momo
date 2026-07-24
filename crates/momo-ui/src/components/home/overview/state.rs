#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct OverviewCarouselState {
    active_card_index: Option<usize>,
}

impl OverviewCarouselState {
    pub(super) fn reconcile(&mut self, card_count: usize, preferred_card_index: Option<usize>) {
        self.active_card_index = match (card_count, self.active_card_index) {
            (0, _) => None,
            (_, Some(active_card_index)) if active_card_index < card_count => {
                Some(active_card_index)
            }
            (_, _) => preferred_card_index
                .filter(|card_index| *card_index < card_count)
                .or(Some(card_count / 2)),
        };
    }

    pub(super) fn active_card_index(self) -> Option<usize> {
        self.active_card_index
    }

    pub(super) fn begin_window_switch(
        &mut self,
        card_count: usize,
        focused_card_index: Option<usize>,
    ) {
        if let Some(focused_card_index) =
            focused_card_index.filter(|card_index| *card_index < card_count)
        {
            self.active_card_index = Some(focused_card_index);
        } else {
            self.reconcile(card_count, None);
        }
    }

    pub(super) fn previous_card_index(self) -> Option<usize> {
        self.active_card_index?.checked_sub(1)
    }

    pub(super) fn next_card_index(self, card_count: usize) -> Option<usize> {
        let next_card_index = self.active_card_index? + 1;
        (next_card_index < card_count).then_some(next_card_index)
    }

    pub(super) fn apply(&mut self, action: OverviewCarouselAction, card_count: usize) {
        match action {
            OverviewCarouselAction::ShowPrevious => {
                if let Some(previous_card_index) = self.previous_card_index() {
                    self.active_card_index = Some(previous_card_index);
                }
            }
            OverviewCarouselAction::ShowNext => {
                if let Some(next_card_index) = self.next_card_index(card_count) {
                    self.active_card_index = Some(next_card_index);
                }
            }
            OverviewCarouselAction::CyclePrevious => {
                self.active_card_index = match (card_count, self.active_card_index) {
                    (0, _) => None,
                    (_, Some(0) | None) => Some(card_count - 1),
                    (_, Some(active_card_index)) => Some(active_card_index - 1),
                };
            }
            OverviewCarouselAction::CycleNext => {
                self.active_card_index = match (card_count, self.active_card_index) {
                    (0, _) => None,
                    (_, Some(active_card_index)) => Some((active_card_index + 1) % card_count),
                    (_, None) => Some(0),
                };
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum OverviewCarouselAction {
    ShowPrevious,
    ShowNext,
    CyclePrevious,
    CycleNext,
}
