#[doc = "Register `CAPCTRL4` reader"]
pub type R = crate::R<Capctrl4Spec>;
#[doc = "Register `CAPCTRL4` writer"]
pub type W = crate::W<Capctrl4Spec>;
#[doc = "Field `CAPCONn_L` reader - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnLR = crate::FieldReader<u16>;
#[doc = "Field `CAPCONn_L` writer - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CAPCONn_H` reader - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnHR = crate::FieldReader<u16>;
#[doc = "Field `CAPCONn_H` writer - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&self) -> CapconnLR {
        CapconnLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&self) -> CapconnHR {
        CapconnHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn capconn_l(&mut self) -> CapconnLW<Capctrl4Spec> {
        CapconnLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn capconn_h(&mut self) -> CapconnHW<Capctrl4Spec> {
        CapconnHW::new(self, 16)
    }
}
#[doc = "SCT capture control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capctrl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capctrl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Capctrl4Spec;
impl crate::RegisterSpec for Capctrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capctrl4::R`](R) reader structure"]
impl crate::Readable for Capctrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`capctrl4::W`](W) writer structure"]
impl crate::Writable for Capctrl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPCTRL4 to value 0"]
impl crate::Resettable for Capctrl4Spec {
    const RESET_VALUE: u32 = 0;
}
