#[doc = "Register `ATLPTDD` reader"]
pub type R = crate::R<AtlptddSpec>;
#[doc = "Register `ATLPTDD` writer"]
pub type W = crate::W<AtlptddSpec>;
#[doc = "Field `ATL_DONE` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type AtlDoneR = crate::FieldReader<u32>;
#[doc = "Field `ATL_DONE` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type AtlDoneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn atl_done(&self) -> AtlDoneR {
        AtlDoneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    #[must_use]
    pub fn atl_done(&mut self) -> AtlDoneW<AtlptddSpec> {
        AtlDoneW::new(self, 0)
    }
}
#[doc = "Done map for each ATL PTD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atlptdd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atlptdd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtlptddSpec;
impl crate::RegisterSpec for AtlptddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atlptdd::R`](R) reader structure"]
impl crate::Readable for AtlptddSpec {}
#[doc = "`write(|w| ..)` method takes [`atlptdd::W`](W) writer structure"]
impl crate::Writable for AtlptddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATLPTDD to value 0"]
impl crate::Resettable for AtlptddSpec {
    const RESET_VALUE: u32 = 0;
}
