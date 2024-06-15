#[doc = "Register `KEYMASK[%s]` writer"]
pub type W = crate::W<KeymaskSpec>;
#[doc = "Field `KEYMASK` writer - Key a Mask"]
pub type KeymaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key a Mask"]
    #[inline(always)]
    #[must_use]
    pub fn keymask(&mut self) -> KeymaskW<KeymaskSpec> {
        KeymaskW::new(self, 0)
    }
}
#[doc = "Key Mask x\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keymask::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeymaskSpec;
impl crate::RegisterSpec for KeymaskSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keymask::W`](W) writer structure"]
impl crate::Writable for KeymaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYMASK[%s]
to value 0"]
impl crate::Resettable for KeymaskSpec {
    const RESET_VALUE: u32 = 0;
}
