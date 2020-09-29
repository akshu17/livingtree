import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.RIYAS_USERNAME)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/MyProfileButton'))

WebUI.delay(70)

WebUI.waitForElementPresent(findTestObject('LivingTree/MyProfilePageLink/MYProfileLeftSide'), 1)

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/ChangeMyPasswordButtom'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyProfilePageLink/ChangePasswordPage'), 1)

WebUI.setText(findTestObject('LivingTree/MyProfilePageLink/CurrentPasswordPasswordBox'), GlobalVariable.PASSWORD)

WebUI.setText(findTestObject('LivingTree/MyProfilePageLink/NewPasswordPasswordBox'), GlobalVariable.PASSWORD)

WebUI.setText(findTestObject('LivingTree/MyProfilePageLink/ConfirmPasswordconfirmPassword'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/ChangePasswordButton'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/EditProfilePicture'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/EditProfilePreferencesButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/ManageCreditCard'))

WebUI.closeBrowser()

