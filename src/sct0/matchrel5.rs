#[doc = "Register `MATCHREL5` reader"]
pub type R = crate::R<Matchrel5Spec>;
#[doc = "Register `MATCHREL5` writer"]
pub type W = crate::W<Matchrel5Spec>;
#[doc = "Field `RELOADn_L` reader - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type ReloadnLR = crate::FieldReader<u16>;
#[doc = "Field `RELOADn_L` writer - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type ReloadnLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RELOADn_H` reader - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type ReloadnHR = crate::FieldReader<u16>;
#[doc = "Field `RELOADn_H` writer - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub type ReloadnHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&self) -> ReloadnLR {
        ReloadnLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&self) -> ReloadnHR {
        ReloadnHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    #[must_use]
    pub fn reloadn_l(&mut self) -> ReloadnLW<Matchrel5Spec> {
        ReloadnLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    #[must_use]
    pub fn reloadn_h(&mut self) -> ReloadnHW<Matchrel5Spec> {
        ReloadnHW::new(self, 16)
    }
}
#[doc = "SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Matchrel5Spec;
impl crate::RegisterSpec for Matchrel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matchrel5::R`](R) reader structure"]
impl crate::Readable for Matchrel5Spec {}
#[doc = "`write(|w| ..)` method takes [`matchrel5::W`](W) writer structure"]
impl crate::Writable for Matchrel5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATCHREL5 to value 0"]
impl crate::Resettable for Matchrel5Spec {
    const RESET_VALUE: u32 = 0;
}
