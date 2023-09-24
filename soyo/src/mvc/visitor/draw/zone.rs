use crate::{
    gfx::{Rect, RectIter, Slot},
    view::{Attribute, Symbol},
};
use std::collections::HashMap;

type ZoneMap = HashMap<(i32, i32), Slot>;

pub struct Zone {
    map: ZoneMap,
    rect: Rect,
    attr: Attribute,
}

impl Zone {
    pub fn new(attr: Attribute) -> Self {
        Self {
            rect: attr.frame.rect(),
            map: ZoneMap::new(),
            attr,
        }
    }

    pub fn collect<I>(&mut self, symbols: I)
    where
        I: IntoIterator<Item = Symbol>,
    {
        let Self {
            rect, map, attr, ..
        } = self;
        for symbol in symbols {
            let slot = symbol.to_slot(attr);
            if rect.point_inside(slot.x, slot.y) {
                map.insert((symbol.x, symbol.y), slot);
            }
        }
    }
}

impl IntoIterator for Zone {
    type Item = Slot;
    type IntoIter = ZoneIter;

    fn into_iter(self) -> Self::IntoIter {
        ZoneIter::new(self)
    }
}

pub struct ZoneIter {
    zone: Zone,
    iter: RectIter,
}

impl ZoneIter {
    pub fn new(zone: Zone) -> Self {
        Self {
            iter: zone.rect.iter(false),
            zone,
        }
    }
}

impl Iterator for ZoneIter {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, zone } = self;
        for (x, y) in iter.by_ref() {
            if let Some(slot) = zone.map.remove(&(x, y)) {
                if slot.c != '\0' {
                    return Some(slot);
                }
            } else if zone.attr.fill {
                let symbol = Symbol::new(x, y, ' ').to_slot(&zone.attr);
                return Some(symbol);
            }
        }
        None
    }
}
